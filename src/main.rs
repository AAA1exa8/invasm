mod disasm;
mod emu;

use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use crate::graphics::Transformed;
use glutin_window::GlutinWindow as Window;
use graphics::{clear, rectangle};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{ButtonEvent, EventLoop};

struct ConditionalFlags {
    z: u8,
    s: u8,
    p: u8,
    cy: u8,
    ac: u8,
    // pad: u8,
}

struct MachineState {
    _read1: u8,
    _read2: u8,
    shift_offset: u8,
    shift0: u8,
    shift1: u8,
    // which_interrupt: u8,
}

pub struct State8080 {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: usize,
    memory: [u8; 0x10000],
    cc: ConditionalFlags,
    int_enable: u8,
}

impl State8080 {
    fn new() -> Self {
        State8080 {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            memory: [0; 0x10000],
            cc: ConditionalFlags {
                z: 0,
                s: 0,
                p: 0,
                cy: 0,
                ac: 0,
                // pad: 0,
            },
            int_enable: 0,
        }
    }

    fn load_into_memory_at(&mut self, offset: usize, file: &str) {
        let mut f = File::open(file).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        for (i, &byte) in buffer.iter().enumerate() {
            self.memory[i + offset] = byte;
        }
    }
}

fn main() {

    let mut state = State8080::new();
    state.load_into_memory_at(0, "invaders.h");
    state.load_into_memory_at(0x800, "invaders.g");
    state.load_into_memory_at(0x1000, "invaders.f");
    state.load_into_memory_at(0x1800, "invaders.e");
    // state.load_into_memory_at(0, "cpudiag.bin");

    let machine_state = Arc::new(Mutex::new(MachineState {
        _read1: 0,
        _read2: 0,
        shift_offset: 0,
        shift0: 0,
        shift1: 0,
        // which_interrupt: 1,
    }));

    let ms = machine_state.clone();

    let display = Arc::new(Mutex::new([0; 256 / 8 * 224]));
    let d = display.clone();

    let emulation_task = std::thread::spawn(move || {
        emulation_loop(&mut state, ms, d);
    });

    render_loop(machine_state, display);

    emulation_task.join().unwrap();
}

fn machine_out(a: u8, port: u8, ports: Arc<Mutex<MachineState>>) {
    match port {
        2 => {
            ports.lock().unwrap().shift_offset = a & 0x7;
        }
        4 => {
            ports.lock().unwrap().shift0 = ports.lock().unwrap().shift1;
            ports.lock().unwrap().shift1 = a;
        }
        i => {} // println!("unimplemented port write {i}"),
    }
}

fn generate_interupt(state: &mut State8080, interrupt_num: u8) {
    state.int_enable = 0;
    state.memory[state.sp as usize - 1] = ((state.pc & 0xff00) >> 8) as u8;
    state.memory[state.sp as usize - 2] = (state.pc & 0xff) as u8;
    state.sp -= 2;
    state.pc = 8 * interrupt_num as usize;
}

fn emulation_loop(
    state: &mut State8080,
    machine_state: Arc<Mutex<MachineState>>,
    display: Arc<Mutex<[u8; 256 / 8 * 224]>>,
) {
    let mut i = 0;
    let mut last_timer = Duration::default();
    let mut next_interrupt = Duration::default();
    let mut which_interrupt = 0;
    let now = Instant::now();

    const CPU_FREQ: u32 = 2_000_000;
    loop {
        if last_timer.as_micros() == 0 {
            last_timer = now.elapsed();
            next_interrupt = last_timer + Duration::from_micros(16000);
            which_interrupt = 1;
        }

        if state.int_enable == 1 && now.elapsed() >= next_interrupt {
            // println!("interrupt");
            if which_interrupt == 1 {
                generate_interupt(state, 1);
                which_interrupt = 2;
            } else {
                generate_interupt(state, 2);
                which_interrupt = 1;
            }
            next_interrupt = now.elapsed() + Duration::from_micros(8000);
        }
        {
            let mut dis = display.lock().unwrap();
            dis.clone_from_slice(state.memory[0x2400..0x2400 + 256 / 8 * 224].as_ref());
        }

        let since_last = now.elapsed() - last_timer;
        let cycles_to_catch_up = CPU_FREQ as u128 * since_last.as_micros() / 1_000_000;
        let mut cycles: u128 = 0;

        while cycles_to_catch_up > cycles {
            // if cycles % 100 == 0 && cycles != 0{
            //     println!("cycles: {}", cycles);
            // }
            let opcode = state.memory[state.pc];
            if opcode == 0xdb {
                let port = state.memory[state.pc + 1];
                state.a = match port {
                    0 => 1, //machine_state.lock().unwrap().read1,
                    1 => 0, //machine_state.lock().unwrap().read2,
                    3 => {
                        let mut ret = (machine_state.lock().unwrap().shift1 as u16).wrapping_shl(8)
                            | (machine_state.lock().unwrap().shift0 as u16);
                        ret = ret
                            .wrapping_shr(8 - machine_state.lock().unwrap().shift_offset as u32)
                            & 0xff;
                        ret as u8
                    }
                    _ => 0,
                };
                state.pc += 2;
                cycles += 3;
            } else if opcode == 0xd3 {
                let port = state.memory[state.pc + 1];
                machine_out(state.a, port, machine_state.clone());
                state.pc += 2;
                cycles += 3;
            } else {
                cycles += emu::emulate_8080(state, i) as u128;
            }
            i += 1;
        }
        last_timer = now.elapsed();
    }
}

fn render_loop(_machine_state: Arc<Mutex<MachineState>>, display: Arc<Mutex<[u8; 256 / 8 * 224]>>) {
    let opengl = OpenGL::V3_2;
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    let square = rectangle::square(0.0, 0.0, 3.0);

    let mut window: Window = WindowSettings::new("Space Invaders", [224 * 3, 256 * 3])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new()).ups(60);

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            let display_array;
            {
                display_array = *display.lock().unwrap();
            }
            gl.draw(args.viewport(), |c, gl| {
                clear(BLACK, gl);
                for y in (0..224).rev() {
                    for x in (0..32).rev() {
                        let address = y * 32 + x;
                        let byte = display_array[address];

                        for i in 0..8 {
                            let byte = byte.wrapping_shr(i as u32);
                            let pixel = byte & 1;

                            let color;
                            if pixel  != 0 {
                                color = WHITE;
                            } else {
                                color = BLACK;
                            }
                            rectangle(color,
                            square,
                            c.transform.trans(224.0*3.0 - ((224 - y) * 3) as f64,256.0*3.0 - (x*8*3 + i*3) as f64),
                            gl);
                        }
                    }
                }

                // for i in 0..224 / 8 {
                //     for j in (0..256).rev() {
                //         let mut color;
                //         for k in 0..8 {
                //             if display_array[j + 256 * i] & 1.shl(k) as u8 != 0 {
                //                 color = WHITE;
                //             } else {
                //                 color = BLACK;
                //             }
                //             rectangle(
                //                 color,
                //                 square,
                //                 c.transform.trans(((8 * i + k) * 3) as f64, (j * 3) as f64),
                //                 gl,
                //             );
                //         }
                //     }
                // }
            })
        }
        if let Some(_args) = e.update_args() {
            // do update
        }
        if let Some(_args) = e.button_args() {
            // do button
        }
    }
}
