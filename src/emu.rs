use crate::disasm::disassemble_8080_op;
use crate::State8080;

const CYCLES: [u8; 256] = [
    4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4, //0x00..0x0f
    4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4, //0x10..0x1f
    4, 10, 16, 5, 5, 5, 7, 4, 4, 10, 16, 5, 5, 5, 7, 4, //etc
    4, 10, 13, 5, 10, 10, 10, 4, 4, 10, 13, 5, 5, 5, 7, 4, 5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5,
    5, 7, 5, //0x40..0x4f
    5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5,
    7, 7, 7, 7, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 7, 5, 4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7,
    4, //0x80..8x4f
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, 11, 10, 10, 10, 17, 11, 7, 11, 11, 10, 10, 10,
    10, 17, 7, 11, //0xc0..0xcf
    11, 10, 10, 10, 17, 11, 7, 11, 11, 10, 10, 10, 10, 17, 7, 11, 11, 10, 10, 18, 17, 11, 7, 11,
    11, 5, 10, 5, 17, 17, 7, 11, 11, 10, 10, 4, 17, 11, 7, 11, 11, 5, 10, 4, 17, 17, 7, 11,
];

pub fn emulate_8080(state: &mut State8080, _iteration: i32) -> u8 {
    let code = state.memory[state.pc];
    // let state.memory = &mut state.memory;
    // disassemble_8080_op(&state.memory, state.pc);
    state.pc += 1;

    // if state.pc == 0x08f4 {
    //     println!("printing");
    //     disassemble_8080_op(&state.memory, state.pc-1);
    // }

    match code {
        0x00 => {} // NOP
        0x01 => {
            // LXI B,b16
            state.c = state.memory[state.pc];
            state.b = state.memory[state.pc + 1];
            state.pc += 2;
        }
        0x02 => {
            // STAX B
            let offset = (state.b as u16).wrapping_shl(8) | (state.c as u16);
            write_mem(state, offset, state.a);
        }
        0x03 => {
            // INX B
            let res = (state.b as u16).wrapping_shl(8) | (state.c as u16);
            let res = res.wrapping_add(1);
            state.b = ((res & 0xff00).wrapping_shr(8)) as u8;
            state.c = (res & 0xff) as u8;
        }
        0x04 => unimplemented_instruction(),
        0x05 => {
            // DCR B
            state.b = state.b.wrapping_sub(1);
            set_flags(state, state.b)
        }
        0x06 => {
            // MVI B, b8
            state.b = state.memory[state.pc];
            state.pc += 1;
        }
        0x07 => {
            // RLC
            let a = state.a;
            state.a = a.rotate_left(1);
            state.cc.cy = if (a & 0x80) != 0 { 1 } else { 0 };
        },
        0x08 => unimplemented_instruction(),
        0x09 => {
            // DAD B
            let hl = (state.h as u32).wrapping_shl(8) | (state.l as u32);
            let bc = (state.b as u32).wrapping_shl(8) | (state.c as u32);
            let res = hl.wrapping_add(bc);
            state.h = ((res & 0xff00).wrapping_shr(8)) as u8;
            state.l = (res & 0xff) as u8;
            state.cc.cy = if (res & 0xffff0000) != 0 { 1 } else { 0 };
        }
        0x0a => {
            // LDAX B
            let offset = (state.b as u32).wrapping_shl(8) | (state.c as u32);
            state.a = state.memory[offset as usize];
        }
        0x0b => unimplemented_instruction(),
        0x0c => unimplemented_instruction(),
        0x0d => {
            // DCR C
            state.c = state.c.wrapping_sub(1);
            set_flags(state, state.c)
        }
        0x0e => {
            // MVI C, b8
            state.c = state.memory[state.pc];
            state.pc += 1;
        }
        0x0f => {
            // RRC
            let x = state.a;
            state.a = x.rotate_right(1);
            state.cc.cy = x & 1;
        }
        0x10 => unimplemented_instruction(),
        0x11 => {
            // LXI D, b16
            state.e = state.memory[state.pc];
            state.d = state.memory[state.pc + 1];
            state.pc += 2;
        }
        0x12 => unimplemented_instruction(),
        0x13 => {
            // INX D
            state.e = state.e.wrapping_add(1);
            if state.e == 0 {
                state.d = state.d.wrapping_add(1);
            }
        }
        0x14 => unimplemented_instruction(),
        0x15 => unimplemented_instruction(),
        0x16 => {
            // MVI D, b8
            state.d = state.memory[state.pc];
            state.pc += 1;
        },
        0x17 => unimplemented_instruction(),
        0x18 => unimplemented_instruction(),
        0x19 => {
            // DAD D
            let hl = (state.h as u32).wrapping_shl(8) | (state.l as u32);
            let de = (state.d as u32).wrapping_shl(8) | (state.e as u32);
            let res = hl.wrapping_add(de);
            state.h = ((res & 0xff00).wrapping_shr(8)) as u8;
            state.l = (res & 0xff) as u8;
            state.cc.cy = if (res & 0xffff0000) != 0 { 1 } else { 0 };
        }
        0x1a => {
            // LDAX D
            let offset = (state.d as u16).wrapping_shl(8) | (state.e as u16);
            state.a = state.memory[offset as usize];
        }
        0x1b => unimplemented_instruction(),
        0x1c => unimplemented_instruction(),
        0x1d => unimplemented_instruction(),
        0x1e => unimplemented_instruction(),
        0x1f => {
            // RAR
            let x = state.a;
            state.a = (x >> 1) | (state.cc.cy << 7);
            state.cc.cy = x & 1;
        },
        0x20 => unimplemented_instruction(),
        0x21 => {
            // LXI H, b16
            state.l = state.memory[state.pc];
            state.h = state.memory[state.pc + 1];
            state.pc += 2;
        }
        0x22 => unimplemented_instruction(),
        0x23 => {
            // INX H
            state.l = state.l.wrapping_add(1);
            if state.l == 0 {
                state.h = state.h.wrapping_add(1);
            }
        }
        0x24 => unimplemented_instruction(),
        0x25 => unimplemented_instruction(),
        0x26 => {
            // MVI H, b8
            state.h = state.memory[state.pc];
            state.pc += 1;
        }
        0x27 => unimplemented_instruction(),
        0x28 => unimplemented_instruction(),
        0x29 => {
            // DAD H
            let hl = (state.h as u32).wrapping_shl(8) | (state.l as u32);
            let res = hl.wrapping_add(hl);
            state.h = ((res & 0xff00).wrapping_shr(8)) as u8;
            state.l = (res & 0xff) as u8;
            state.cc.cy = if (res & 0xffff0000) != 0 { 1 } else { 0 };
        }
        0x2a => {
            // LHLD adr
            let offset = (state.memory[state.pc + 1] as u16).wrapping_shl(8)
                | (state.memory[state.pc] as u16);
            state.l = state.memory[offset as usize];
            state.h = state.memory[(offset + 1) as usize];
            state.pc += 2;

        },
        0x2b => unimplemented_instruction(),
        0x2c => unimplemented_instruction(),
        0x2d => unimplemented_instruction(),
        0x2e => {
            // MVI L, b8
            state.l = state.memory[state.pc];
            state.pc += 1;
        }
        0x2f => unimplemented_instruction(),
        0x30 => unimplemented_instruction(),
        0x31 => {
            // LXI SP, b16
            state.sp = (state.memory[state.pc + 1] as u16).wrapping_shl(8)
                | (state.memory[state.pc] as u16);
            state.pc += 2;
        }
        0x32 => {
            // STA adr
            let offset = (state.memory[state.pc + 1] as u16).wrapping_shl(8)
                | (state.memory[state.pc] as u16);
            write_mem(state, offset, state.a);
            state.pc += 2;
        }
        0x33 => unimplemented_instruction(),
        0x34 => {
            // INR M
            let res = read_from_hl(state).wrapping_add(1);
            write_to_hl(state, res);
            set_flags(state, res);
        },
        0x35 => {
            // DCR M
            let res = read_from_hl(state).wrapping_sub(1);
            write_to_hl(state, res);
            set_flags(state, res);
        }
        0x36 => {
            // MVI M, b8
            write_to_hl(state, state.memory[state.pc]);
            state.pc += 1;
        }
        0x37 => {
            // STC
            state.cc.cy = 1;
        }
        0x38 => unimplemented_instruction(),
        0x39 => unimplemented_instruction(),
        0x3a => {
            // LDA adr
            let offset = (state.memory[state.pc + 1] as u16).wrapping_shl(8)
                | (state.memory[state.pc] as u16);
            state.a = state.memory[offset as usize];
            state.pc += 2;
        }
        0x3b => unimplemented_instruction(),
        0x3c => {
            // INR A
            state.a = state.a.wrapping_add(1);
            set_flags(state, state.a);
        },
        0x3d => {
            // DCR A
            state.a = state.a.wrapping_sub(1);
            set_flags(state, state.a);
        }
        0x3e => {
            // MVI A, b8
            state.a = state.memory[state.pc];
            state.pc += 1;
        }
        0x3f => unimplemented_instruction(),
        0x40 => unimplemented_instruction(),
        0x41 => unimplemented_instruction(),
        0x42 => unimplemented_instruction(),
        0x43 => unimplemented_instruction(),
        0x44 => unimplemented_instruction(),
        0x45 => unimplemented_instruction(),
        0x46 => {
            // MOV M,B
            state.b = read_from_hl(state);
        }
        0x47 => unimplemented_instruction(),
        0x48 => unimplemented_instruction(),
        0x49 => unimplemented_instruction(),
        0x4a => unimplemented_instruction(),
        0x4b => unimplemented_instruction(),
        0x4c => unimplemented_instruction(),
        0x4d => unimplemented_instruction(),
        0x4e => unimplemented_instruction(),
        0x4f => {
            // MOV C,A
            state.c = state.a;
        }
        0x50 => unimplemented_instruction(),
        0x51 => unimplemented_instruction(),
        0x52 => unimplemented_instruction(),
        0x53 => unimplemented_instruction(),
        0x54 => unimplemented_instruction(),
        0x55 => unimplemented_instruction(),
        0x56 => {
            // MOV D, M
            state.d = read_from_hl(state);
        }
        0x57 => {
            // MOV D, A
            state.d = state.a;
        }
        0x58 => unimplemented_instruction(),
        0x59 => unimplemented_instruction(),
        0x5a => unimplemented_instruction(),
        0x5b => unimplemented_instruction(),
        0x5c => unimplemented_instruction(),
        0x5d => {
            // MOV E, L
            state.e = state.l;
        }
        0x5e => {
            // MOV E, M
            state.e = read_from_hl(state);
        }
        0x5f => {
            // MOV E, A
            state.e = state.a;
        }
        0x60 => unimplemented_instruction(),
        0x61 => unimplemented_instruction(),
        0x62 => unimplemented_instruction(),
        0x63 => unimplemented_instruction(),
        0x64 => unimplemented_instruction(),
        0x65 => unimplemented_instruction(),
        0x66 => {
            // MOV H, M
            state.h = read_from_hl(state);
        }
        0x67 => {
            // MOV H, A
            state.h = state.a;
        }
        0x68 => unimplemented_instruction(),
        0x69 => unimplemented_instruction(),
        0x6a => unimplemented_instruction(),
        0x6b => unimplemented_instruction(),
        0x6c => unimplemented_instruction(),
        0x6d => unimplemented_instruction(),
        0x6e => unimplemented_instruction(),
        0x6f => {
            // MOV L, A
            state.l = state.a;
        }
        0x70 => unimplemented_instruction(),
        0x71 => unimplemented_instruction(),
        0x72 => unimplemented_instruction(),
        0x73 => unimplemented_instruction(),
        0x74 => unimplemented_instruction(),
        0x75 => unimplemented_instruction(),
        0x76 => unimplemented_instruction(),
        0x77 => {
            // MOV M, A
            write_to_hl(state, state.a);
        }
        0x78 => {
            // MOV A, B
            state.a = state.b;
        },
        0x79 => {
            // MOV A, C
            state.a = state.c;
        }
        0x7a => {
            // MOV D, A
            state.a = state.d;
        }
        0x7b => {
            // MOV E, A
            state.a = state.e;
        }
        0x7c => {
            // MOV H, A
            state.a = state.h;
        }
        0x7d => {
            // MOV L, A
            state.a = state.l;
        }
        0x7e => {
            // MOV A, M
            state.a = read_from_hl(state);
        }
        0x7f => unimplemented_instruction(),
        0x80 => unimplemented_instruction(),
        0x81 => unimplemented_instruction(),
        0x82 => unimplemented_instruction(),
        0x83 => unimplemented_instruction(),
        0x84 => unimplemented_instruction(),
        0x85 => unimplemented_instruction(),
        0x86 => unimplemented_instruction(),
        0x87 => unimplemented_instruction(),
        0x88 => unimplemented_instruction(),
        0x89 => unimplemented_instruction(),
        0x8a => unimplemented_instruction(),
        0x8b => unimplemented_instruction(),
        0x8c => unimplemented_instruction(),
        0x8d => unimplemented_instruction(),
        0x8e => unimplemented_instruction(),
        0x8f => unimplemented_instruction(),
        0x90 => unimplemented_instruction(),
        0x91 => unimplemented_instruction(),
        0x92 => unimplemented_instruction(),
        0x93 => unimplemented_instruction(),
        0x94 => unimplemented_instruction(),
        0x95 => unimplemented_instruction(),
        0x96 => unimplemented_instruction(),
        0x97 => unimplemented_instruction(),
        0x98 => unimplemented_instruction(),
        0x99 => unimplemented_instruction(),
        0x9a => unimplemented_instruction(),
        0x9b => unimplemented_instruction(),
        0x9c => unimplemented_instruction(),
        0x9d => unimplemented_instruction(),
        0x9e => unimplemented_instruction(),
        0x9f => unimplemented_instruction(),
        0xa0 => unimplemented_instruction(),
        0xa1 => unimplemented_instruction(),
        0xa2 => unimplemented_instruction(),
        0xa3 => unimplemented_instruction(),
        0xa4 => unimplemented_instruction(),
        0xa5 => unimplemented_instruction(),
        0xa6 => unimplemented_instruction(),
        0xa7 => {
            // ANA A
            // state.a = state.a & state.a;
            set_flags_a(state);
        }
        0xa8 => {
            // XRA B
            state.a = state.a ^ state.b;
            set_flags_a(state);
        },
        0xa9 => unimplemented_instruction(),
        0xaa => unimplemented_instruction(),
        0xab => unimplemented_instruction(),
        0xac => unimplemented_instruction(),
        0xad => unimplemented_instruction(),
        0xae => unimplemented_instruction(),
        0xaf => {
            // XRA A
            state.a = 0;
            set_flags_a(state);
        }
        0xb0 => {
            // ORA B
            state.a = state.a | state.b;
            set_flags_a(state);
        },
        0xb1 => unimplemented_instruction(),
        0xb2 => unimplemented_instruction(),
        0xb3 => unimplemented_instruction(),
        0xb4 => unimplemented_instruction(),
        0xb5 => unimplemented_instruction(),
        0xb6 => {
            // ORA M
            state.a = state.a | read_from_hl(state);
            set_flags_a(state);
        },
        0xb7 => unimplemented_instruction(),
        0xb8 => unimplemented_instruction(),
        0xb9 => unimplemented_instruction(),
        0xba => unimplemented_instruction(),
        0xbb => unimplemented_instruction(),
        0xbc => unimplemented_instruction(),
        0xbd => unimplemented_instruction(),
        0xbe => unimplemented_instruction(),
        0xbf => unimplemented_instruction(),
        0xc0 => {
            // RNZ
            if state.cc.z == 0 {
                state.pc = state.memory[state.sp as usize] as usize
                    | (state.memory[(state.sp + 1) as usize].wrapping_shl(8)) as usize;
                state.sp += 2;
            }
        }
        0xc1 => {
            // POP B
            state.c = state.memory[state.sp as usize];
            state.b = state.memory[(state.sp + 1) as usize];
            state.sp += 2;
        }
        0xc2 => {
            // JNZ adr
            if state.cc.z == 0 {
                state.pc = (((state.memory[state.pc + 1] as u16).wrapping_shl(8))
                    | state.memory[state.pc] as u16) as usize;
            } else {
                state.pc += 2;
            }
        }
        0xc3 => {
            // JMP adr
            state.pc = (((state.memory[state.pc + 1] as u16).wrapping_shl(8))
                | state.memory[state.pc] as u16) as usize;
        }
        0xc4 => {
            // CNZ adr
            if state.cc.z == 0 {
                let pc = state.pc + 2;
                state.memory[state.sp as usize - 1] = ((pc >> 8) & 0xff) as u8;
                state.memory[state.sp as usize - 2] = (pc & 0xff) as u8;
                state.sp = state.sp - 2;
                state.pc = (((state.memory[state.pc + 1] as u16).wrapping_shl(8))
                    | state.memory[state.pc] as u16) as usize;
            } else {
                state.pc += 2;
            }
        },
        0xc5 => {
            // PUSH B
            push_stack(state, state.b, state.c);
        }
        0xc6 => {
            // ADI D8
            let x = state.a as u16 + state.memory[state.pc] as u16;
            set_flags_a(state);
            state.cc.cy = if x > 0xff { 1 } else { 0 };
            state.a = x as u8;
            state.pc += 1;
        }
        0xc7 => unimplemented_instruction(),
        0xc8 => {
            // RZ
            if state.cc.z == 1 {
                state.pc = (((state.memory[(state.sp + 1) as usize] as u16).wrapping_shl(8))
                    | state.memory[state.sp as usize] as u16) as usize;
                state.sp += 2;
            }
        }
        0xc9 => {
            // RET
            state.pc = (((state.memory[(state.sp + 1) as usize] as u16).wrapping_shl(8))
                | state.memory[state.sp as usize] as u16) as usize;
            state.sp += 2;
        }
        0xca => {
            // JZ D16
            if state.cc.z == 1 {
                state.pc = (((state.memory[state.pc + 1] as u16).wrapping_shl(8))
                    | state.memory[state.pc] as u16) as usize;
            } else {
                state.pc += 2;
            }
        }
        0xcb => unimplemented_instruction(),
        0xcc => {
            // CZ D16
            if state.cc.z == 1 {
                let ret = state.pc + 2;
                let a = ((ret.wrapping_shr(8)) & 0xff) as u8;
                let h = (ret & 0xff) as u8;
                write_mem(state, state.sp - 1, a);
                write_mem(state, state.sp - 2, h);
                state.sp -= 2;
                state.pc = (((state.memory[state.pc + 1] as u16).wrapping_shl(8))
                    | state.memory[state.pc] as u16) as usize;
            } else {
                state.pc += 2;
            }
        }
        0xcd => {
            // CALL adr
            let ret = state.pc + 2;
            let a = ((ret.wrapping_shr(8)) & 0xff) as u8;
            let h = (ret & 0xff) as u8;
            write_mem(state, state.sp - 1, a);
            write_mem(state, state.sp - 2, h);
            state.sp -= 2;
            state.pc = (((state.memory[state.pc + 1] as u16).wrapping_shl(8))
                | state.memory[state.pc] as u16) as usize;
        }
        0xce => unimplemented_instruction(),
        0xcf => unimplemented_instruction(),
        0xd0 => {
            // RNC
            if state.cc.cy == 0 {
                state.pc = (((state.memory[(state.sp + 1) as usize] as u16).wrapping_shl(8))
                    | state.memory[state.sp as usize] as u16) as usize;
                state.sp += 2;
            }
        },
        0xd1 => {
            // POP D
            // state.e = state.memory[state.sp as usize];
            // state.d = state.memory[(state.sp + 1) as usize];
            // state.sp += 2;
            let (high, low) = pop_stack(state);
            state.d = high;
            state.e = low;
        }
        0xd2 => {
            // JNC D16
            if state.cc.cy == 0 {
                state.pc = (((state.memory[state.pc + 1] as u16).wrapping_shl(8))
                    | state.memory[state.pc] as u16) as usize;
            } else {
                state.pc += 2;
            }
        }
        0xd3 => {
            // ! OUT D8
            // let port = state.memory[state.pc] as u16;
            // match port {
            //     2 => state.ports.lock().unwrap().write2 = state.a,
            //     4 => state.ports.lock().unwrap().write4 = state.a,
            //     _ => panic!("Invalid port"),
            // };
            // state.pc += 1;
            unimplemented_instruction();
        }
        0xd4 => unimplemented_instruction(),
        0xd5 => {
            // PUSH D
            push_stack(state, state.d, state.e);
        }
        0xd6 => unimplemented_instruction(),
        0xd7 => unimplemented_instruction(),
        0xd8 => {
            // RC
            if state.cc.cy == 1 {
                state.pc = (((state.memory[(state.sp + 1) as usize] as u16).wrapping_shl(8))
                    | state.memory[state.sp as usize] as u16) as usize;
                state.sp += 2;
            }
        }
        0xd9 => unimplemented_instruction(),
        0xda => {
            // JC
            if state.cc.cy == 1 {
                state.pc = (((state.memory[state.pc + 1] as u16).wrapping_shl(8))
                    | state.memory[state.pc] as u16) as usize;
            } else {
                state.pc += 2;
            }
        }
        0xdb => {
            // IN D8
            // let port = state.memory[state.pc] as u16;
            // state.a = match port {
            //     0 => state.ports.lock().unwrap().read1,
            //     1 => state.ports.lock().unwrap().read2,
            //     2 => state.ports.lock().unwrap().read3,
            //     _ => panic!("Invalid port"),
            // };
            // state.pc += 1;
            unimplemented_instruction();
        }
        0xdc => unimplemented_instruction(),
        0xdd => unimplemented_instruction(),
        0xde => unimplemented_instruction(),
        0xdf => unimplemented_instruction(),
        0xe0 => unimplemented_instruction(),
        0xe1 => {
            // POP H
            let (high, low) = pop_stack(state);
            state.h = high;
            state.l = low;
        }
        0xe2 => unimplemented_instruction(),
        0xe3 => {
            // XTHL
            let l = state.l;
            let h = state.h;
            let sp = state.sp;
            state.l = state.memory[sp as usize];
            state.h = state.memory[sp as usize + 1];
            write_mem(state, sp, l);
            write_mem(state, sp + 1, h);
        },
        0xe4 => unimplemented_instruction(),
        0xe5 => {
            // PUSH H
            push_stack(state, state.h, state.l);
        }
        0xe6 => {
            // ANI D8
            state.a &= state.memory[state.pc];
            set_flags_a(state);
            state.pc += 1;
        }
        0xe7 => unimplemented_instruction(),
        0xe8 => unimplemented_instruction(),
        0xe9 => {
            // PCHL
            state.pc = ((state.h as u16).wrapping_shl(8) | state.l as u16) as usize;
        },
        0xea => unimplemented_instruction(),
        0xeb => {
            // XCHG
            let d = state.d;
            let e = state.e;
            state.d = state.h;
            state.e = state.l;
            state.h = d;
            state.l = e;
        }
        0xec => unimplemented_instruction(),
        0xed => unimplemented_instruction(),
        0xee => unimplemented_instruction(),
        0xef => unimplemented_instruction(),
        0xf0 => unimplemented_instruction(),
        0xf1 => {
            // POP PSW
            state.a = state.memory[(state.sp + 1) as usize];
            let psw = state.memory[state.sp as usize];
            state.cc.z = (psw.wrapping_shr(6)) & 1;
            state.cc.s = (psw.wrapping_shr(7)) & 1;
            state.cc.p = (psw.wrapping_shr(2)) & 1;
            state.cc.cy = (psw.wrapping_shr(0)) & 1;
            state.cc.ac = (psw.wrapping_shr(4)) & 1;
            state.sp += 2;
        }
        0xf2 => unimplemented_instruction(),
        0xf3 => unimplemented_instruction(),
        0xf4 => unimplemented_instruction(),
        0xf5 => {
            // PUSH PSW
            state.memory[(state.sp - 1) as usize] = state.a;
            let mut psw: u8 = 0;
            psw |= (state.cc.z.wrapping_shl(6)) & 0x40;
            psw |= (state.cc.s.wrapping_shl(7)) & 0x80;
            psw |= (state.cc.p.wrapping_shl(2)) & 0x04;
            psw |= (state.cc.cy.wrapping_shl(0)) & 0x01;
            psw |= (state.cc.ac.wrapping_shl(4)) & 0x10;
            state.memory[(state.sp - 2) as usize] = psw;
            state.sp -= 2;
        }
        0xf6 => {
            // ORI D8
            state.a |= state.memory[state.pc];
            set_flags_a(state);
            state.pc += 1;
        },
        0xf7 => unimplemented_instruction(),
        0xf8 => unimplemented_instruction(),
        0xf9 => unimplemented_instruction(),
        0xfa => unimplemented_instruction(),
        0xfb => {
            // EI
            state.int_enable = 1;
        }
        0xfc => unimplemented_instruction(),
        0xfd => unimplemented_instruction(),
        0xfe => {
            // CPI D8
            let x = (state.a).wrapping_sub(state.memory[state.pc]);
            set_flags(state, x);
            state.cc.cy = if state.a < state.memory[state.pc] {
                1
            } else {
                0
            };
            state.pc += 1;
        }
        0xff => unimplemented_instruction(),
        _ => unimplemented_instruction(),
    }

    // println!(
    //     "Flags: Z: {} S: {} P: {} CY: {} AC: {}",
    //     state.cc.z, state.cc.s, state.cc.p, state.cc.cy, state.cc.ac
    // );
    // println!("A: {:02x} B: {:02x} C: {:02x} D: {:02x} E: {:02x} H: {:02x} L: {:02x} SP: {:04x} PC: {:04x} INTERRUPT: {:02x}",
    //          state.a, state.b, state.c, state.d, state.e, state.h, state.l, state.sp, state.pc, state.int_enable);

    CYCLES[code as usize]
}

fn unimplemented_instruction() {
    panic!("Error: Unimplemented instruction");
}

fn parity(x: u8, size: usize) -> u8 {
    let mut x = x as u16;
    let mut p = 0;
    let inter = 1u16.wrapping_shl(size as u32);
    x &= inter - 1;
    for _ in 0..size {
        if x & 0x1 == 1 {
            p += 1;
        }
        x = x.wrapping_shr(1);
    }
    ((p & 0x1) == 0) as u8
}

fn read_from_hl(state: &mut State8080) -> u8 {
    let offset = (state.h as u16).wrapping_shl(8) | (state.l as u16);
    state.memory[offset as usize]
}

fn write_to_hl(state: &mut State8080, value: u8) {
    let offset = (state.h as u16).wrapping_shl(8) | (state.l as u16);
    write_mem(state, offset, value);
}

fn set_flags_a(state: &mut State8080) {
    state.cc.cy = 0;
    state.cc.ac = 0;
    state.cc.z = if state.a == 0 { 1 } else { 0 };
    state.cc.s = if (state.a & 0x80) == 0x80 { 1 } else { 0 };
    state.cc.p = parity(state.a, 8);
}

fn set_flags(state: &mut State8080, value: u8) {
    state.cc.z = if value == 0 { 1 } else { 0 };
    state.cc.s = if (value & 0x80) == 0x80 { 1 } else { 0 };
    state.cc.p = parity(value, 8);
}

fn write_mem(state: &mut State8080, address: u16, value: u8) {
    if address < 0x2000 {
        // ROM
        return;
    }
    if address >= 0x4000 {
        return;
    }
    // if address >= 0x2400 && address <= 0x3fff {
    //     println!("Writing to video memory at {:04x}", address);
    // }
    state.memory[address as usize] = value;
}

fn push_stack(state: &mut State8080, high: u8, low: u8) {
    write_mem(state, state.sp - 1, high);
    write_mem(state, state.sp - 2, low);
    state.sp -= 2;
}

fn pop_stack(state: &mut State8080) -> (u8, u8) {
    let low = state.memory[state.sp as usize];
    let high = state.memory[(state.sp + 1) as usize];
    state.sp += 2;
    (high, low)
}
