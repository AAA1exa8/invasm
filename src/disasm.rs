pub fn disassemble_8080_op(code_buffer: &[u8], pc: usize) -> usize {
    let code = &code_buffer[pc];
    let mut opbytes = 1;
    print!("{:04x} ", pc);

    match *code {
        0x00 => println!("NOP \t\t\t ; no operation"),
        0x01 => {
            println!("LXI    B,#${:02x}{:02x}\t\t ; moves the value #${0:02x}{1:02x} to BC register", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0x02 => println!("STAX   B \t\t\t ; stores the value of A(cumulator) register to the memory location pointed by BC register"),
        0x03 => println!("INX    B \t\t\t ; increments the value of BC register by 1"),
        0x04 => println!("INR    B \t\t\t ; increments the value of B register by 1"),
        0x05 => println!("DCR    B \t\t\t ; decrements the value of B register by 1"),
        0x06 => {
            println!("MVI    B,#${:02x}\t\t ; moves the value #${0:02x} to B register", code_buffer[pc + 1]);
            opbytes = 2;
        }
        0x07 => println!("RLC \t\t\t ; rotates the value of A register to the left by 1 bit (the rightmost bit is copied to the leftmost bit) and CY flag is set to the value of the leftmost bit of A register"),
        0x08 => println!("NOP \t\t\t ; no operation /* (reserved) */"),
        0x09 => println!("DAD    B \t\t\t ; adds the value of BC register to the value of HL register and stores the result in HL register"),
        0x0a => println!("LDAX   B \t\t\t ; loads the value of the memory location pointed by BC register to A(cumulator) register"),
        0x0b => println!("DCX    B \t\t\t ; decrements the value of BC register by 1"),
        0x0c => println!("INR    C \t\t\t ; increments the value of C register by 1"),
        0x0d => println!("DCR    C \t\t\t ; decrements the value of C register by 1"),
        0x0e => {
            println!("MVI    C,#${:02x}\t\t ; moves the value #${0:02x} to C register", code_buffer[pc + 1]);
            opbytes = 2;
        }
        0x0f => println!("RRC \t\t\t ; rotates the value of A register to the right by 1 bit (the leftmost bit is copied to the rightmost bit) and CY flag is set to the value of the rightmost bit of A register"),
        0x10 => println!("NOP \t\t\t ; no operation /* (reserved) */"),
        0x11 => {
            println!("LXI    D,#${:02x}{:02x}\t\t ; moves the value #${0:02x}{1:02x} to DE register", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0x12 => println!("STAX   D \t\t\t ; stores the value of A(cumulator) register to the memory location pointed by DE register"),
        0x13 => println!("INX    D \t\t\t ; increments the value of DE register by 1"),
        0x14 => println!("INR    D \t\t\t ; increments the value of D register by 1"),
        0x15 => println!("DCR    D \t\t\t ; decrements the value of D register by 1"),
        0x16 => {
            println!("MVI    D,#${:02x}\t\t ; moves the value #${0:02x} to D register", code_buffer[pc + 1]);
            opbytes = 2;
        }
        0x17 => println!("RAL \t\t\t ; rotates the value of A register to the left by 1 bit the rightmost bit is set to the value of CY flag and CY flag is set to the value of the leftmost bit of A register"),
        0x18 => println!("NOP \t\t\t ; no operation /* (reserved) */"),
        0x19 => println!("DAD    D \t\t\t ; adds the value of DE register to the value of HL register and stores the result in HL register"),
        0x1a => println!("LDAX   D \t\t\t ; loads the value of the memory location pointed by DE register to A(cumulator) register"),
        0x1b => println!("DCX    D \t\t\t ; decrements the value of DE register by 1"),
        0x1c => println!("INR    E \t\t\t ; increments the value of E register by 1"),
        0x1d => println!("DCR    E \t\t\t ; decrements the value of E register by 1"),
        0x1e => {
            println!("MVI    E,#${:02x}\t\t ; moves the value #${0:02x} to E register", code_buffer[pc + 1]);
            opbytes = 2;
        }
        0x1f => println!("RAR \t\t\t ; rotates the value of A register to the right by 1 bit the leftmost bit is set to the value of CY flag and CY flag is set to the value of the rightmost bit of A register"),
        0x20 => println!("RIM \t\t\t ; special"),
        0x21 => {
            println!("LXI    H,#${:02x}{:02x}\t\t ; moves the value #${0:02x}{1:02x} to HL register", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0x22 => {
            println!("SHLD   ${:02x}{:02x}\t\t ; stores the value of L register to the memory location pointed by the #${0:02x}{1:02x} bytes", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0x23 => println!("INX    H \t\t\t ; increments the value of HL register by 1"),
        0x24 => println!("INR    H \t\t\t ; increments the value of H register by 1"),
        0x25 => println!("DCR    H \t\t\t ; decrements the value of H register by 1"),
        0x26 => {
            println!("MVI    H,#${:02x}\t\t ; moves the value #${0:02x} to H register", code_buffer[pc + 1]);
            opbytes = 2;
        }
        0x27 => println!("DAA \t\t\t ; special"),
        0x28 => println!("NOP \t\t\t ; no operation /* (reserved) */"),
        0x29 => println!("DAD    H \t\t\t ; adds the value of HL register to the value of HL register and stores the result in HL register"),
        0x2a => {
            println!("LHLD   ${:02x}{:02x}\t\t ; loads the value of the memory location pointed by the #${0:02x}{1:02x} bytes to HL register", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0x2b => println!("DCX    H \t\t\t ; decrements the value of HL register by 1"),
        0x2c => println!("INR    L \t\t\t ; increments the value of L register by 1"),
        0x2d => println!("DCR    L \t\t\t ; decrements the value of L register by 1"),
        0x2e => {
            println!("MVI    L,#${:02x}\t\t ; moves the value #${0:02x} to L register", code_buffer[pc + 1]);
            opbytes = 2;
        }
        0x2f => println!("CMA \t\t\t ; inverts value of A register"),
        0x30 => println!("SIM \t\t\t ; special"),
        0x31 => {
            println!("LXI    SP,#${:02x}{:02x}\t\t ; moves the value #${0:02x}{1:02x} to SP register", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0x32 => {
            if pc == 2046 {
                println!("STA    ${:02x}{:02x}\t\t ; stores the value of A register to the memory location pointed by the #${0:02x}{1:02x} bytes", 00, code_buffer[pc + 1]);
            } else {
                println!("STA    ${:02x}{:02x}\t\t ; stores the value of A register to the memory location pointed by the #${0:02x}{1:02x} bytes", code_buffer[pc + 2], code_buffer[pc + 1]);
            }
            opbytes = 3;
        }
        0x33 => println!("INX    SP \t\t\t ; increments the value of SP register by 1"),
        0x34 => println!("INR    M \t\t\t ; increments the value of the memory location pointed by HL register by 1"),
        0x35 => println!("DCR    M \t\t\t ; decrements the value of the memory location pointed by HL register by 1"),
        0x36 => {
            println!("MVI    M,#${:02x}\t\t ; moves the value #${0:02x} to the memory location pointed by HL register", code_buffer[pc + 1]);
            opbytes = 2;
        }
        0x37 => println!("STC \t\t\t ; sets CY flag to 1"),
        0x38 => println!("NOP \t\t\t ; no operation /* (reserved) */"),
        0x39 => println!("DAD    SP \t\t\t ; adds the value of SP register to the value of HL register and stores the result in HL register"),
        0x3a => {
            println!("LDA    ${:02x}{:02x}\t\t ; loads the value of the memory location pointed by the #${0:02x}{1:02x} bytes to A register", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0x3b => println!("DCX    SP \t\t\t ; decrements the value of SP register by 1"),
        0x3c => println!("INR    A \t\t\t ; increments the value of A register by 1"),
        0x3d => println!("DCR    A \t\t\t ; decrements the value of A register by 1"),
        0x3e => {
            println!("MVI    A,#0x{:02x} \t\t ; moves the value #0x{0:02x} to the A(cumulator) register", code_buffer[pc + 1]);
            opbytes = 2;
        }
        0x3f => println!("CMC \t\t\t ; inverts the value of CY flag"),
        // ! empty
        0x40 => println!("MOV    B,B \t\t ; copies the value of B register to B register"),
        0x41 => println!("MOV    B,C \t\t ; copies the value of C register to B register"),
        0x42 => println!("MOV    B,D \t\t ; copies the value of D register to B register"),
        0x43 => println!("MOV    B,E \t\t ; copies the value of E register to B register"),
        0x44 => println!("MOV    B,H \t\t ; copies the value of H register to B register"),
        0x45 => println!("MOV    B,L \t\t ; copies the value of L register to B register"),
        0x46 => println!("MOV    B,M \t\t ; copies the value of the memory location pointed by HL register to B register"),
        0x47 => println!("MOV    B,A \t\t ; copies the value of A(cumulator) register to B register"),
        0x48 => println!("MOV    C,B \t\t ; copies the value of B register to C register"),
        0x49 => println!("MOV    C,C \t\t ; copies the value of C register to C register"),
        0x4a => println!("MOV    C,D \t\t ; copies the value of D register to C register"),
        0x4b => println!("MOV    C,E \t\t ; copies the value of E register to C register"),
        0x4c => println!("MOV    C,H \t\t ; copies the value of H register to C register"),
        0x4d => println!("MOV    C,L \t\t ; copies the value of L register to C register"),
        0x4e => println!("MOV    C,M \t\t ; copies the value of the memory location pointed by HL register to C register"),
        0x4f => println!("MOV    C,A \t\t ; copies the value of A(cumulator) register to C register"),
        0x50 => println!("MOV    D,B \t\t ; copies the value of B register to D register"),
        0x51 => println!("MOV    D,C \t\t ; copies the value of C register to D register"),
        0x52 => println!("MOV    D,D \t\t ; copies the value of D register to D register"),
        0x53 => println!("MOV    D,E \t\t ; copies the value of E register to D register"),
        0x54 => println!("MOV    D,H \t\t ; copies the value of H register to D register"),
        0x55 => println!("MOV    D,L \t\t ; copies the value of L register to D register"),
        0x56 => println!("MOV    D,M \t\t ; copies the value of the memory location pointed by HL register to D register"),
        0x57 => println!("MOV    D,A \t\t ; copies the value of A(cumulator) register to D register"),
        0x58 => println!("MOV    E,B \t\t ; copies the value of B register to E register"),
        0x59 => println!("MOV    E,C \t\t ; copies the value of C register to E register"),
        0x5a => println!("MOV    E,D \t\t ; copies the value of D register to E register"),
        0x5b => println!("MOV    E,E \t\t ; copies the value of E register to E register"),
        0x5c => println!("MOV    E,H \t\t ; copies the value of H register to E register"),
        0x5d => println!("MOV    E,L \t\t ; copies the value of L register to E register"),
        0x5e => println!("MOV    E,M \t\t ; copies the value of the memory location pointed by HL register to E register"),
        0x5f => println!("MOV    E,A \t\t ; copies the value of A(cumulator) register to E register"),
        0x60 => println!("MOV    H,B \t\t ; copies the value of B register to H register"),
        0x61 => println!("MOV    H,C \t\t ; copies the value of C register to H register"),
        0x62 => println!("MOV    H,D \t\t ; copies the value of D register to H register"),
        0x63 => println!("MOV    H,E \t\t ; copies the value of E register to H register"),
        0x64 => println!("MOV    H,H \t\t ; copies the value of H register to H register"),
        0x65 => println!("MOV    H,L \t\t ; copies the value of L register to H register"),
        0x66 => println!("MOV    H,M \t\t ; copies the value of the memory location pointed by HL register to H register"),
        0x67 => println!("MOV    H,A \t\t ; copies the value of A(cumulator) register to H register"),
        0x68 => println!("MOV    L,B \t\t ; copies the value of B register to L register"),
        0x69 => println!("MOV    L,C \t\t ; copies the value of C register to L register"),
        0x6a => println!("MOV    L,D \t\t ; copies the value of D register to L register"),
        0x6b => println!("MOV    L,E \t\t ; copies the value of E register to L register"),
        0x6c => println!("MOV    L,H \t\t ; copies the value of H register to L register"),
        0x6d => println!("MOV    L,L \t\t ; copies the value of L register to L register"),
        0x6e => println!("MOV    L,M \t\t ; copies the value of the memory location pointed by HL register to L register"),
        0x6f => println!("MOV    L,A \t\t ; copies the value of A(cumulator) register to L register"),
        // ! empty
        0x70 => println!("MOV    M,B \t\t ; copies the value of B register to the memory location pointed by HL register"),
        0x71 => println!("MOV    M,C \t\t ; copies the value of C register to the memory location pointed by HL register"),
        0x72 => println!("MOV    M,D \t\t ; copies the value of D register to the memory location pointed by HL register"),
        0x73 => println!("MOV    M,E \t\t ; copies the value of E register to the memory location pointed by HL register"),
        0x74 => println!("MOV    M,H \t\t ; copies the value of H register to the memory location pointed by HL register"),
        0x75 => println!("MOV    M,L \t\t ; copies the value of L register to the memory location pointed by HL register"),
        0x76 => println!("HLT \t\t\t ; special"),
        0x77 => println!("MOV    M,A \t\t ; copies the value of A(cumulator) register to the memory location pointed by HL register"),
        0x78 => println!("MOV    A,B \t\t ; copies the value of B register to A(cumulator) register"),
        0x79 => println!("MOV    A,C \t\t ; copies the value of C register to A(cumulator) register"),
        0x7a => println!("MOV    A,D \t\t ; copies the value of D register to A(cumulator) register"),
        0x7b => println!("MOV    A,E \t\t ; copies the value of E register to A(cumulator) register"),
        0x7c => println!("MOV    A,H \t\t ; copies the value of H register to A(cumulator) register"),
        0x7d => println!("MOV    A,L \t\t ; copies the value of L register to A(cumulator) register"),
        0x7e => println!("MOV    A,M \t\t ; copies the value of the memory location pointed by HL register to A(cumulator) register"),
        0x7f => println!("MOV    A,A \t\t ; copies the value of A(cumulator) register to A(cumulator) register"),
        0x80 => println!("ADD    B \t\t\t ; adds the value of B register to the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x81 => println!("ADD    C \t\t\t ; adds the value of C register to the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x82 => println!("ADD    D \t\t\t ; adds the value of D register to the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x83 => println!("ADD    E \t\t\t ; adds the value of E register to the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x84 => println!("ADD    H \t\t\t ; adds the value of H register to the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x85 => println!("ADD    L \t\t\t ; adds the value of L register to the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x86 => println!("ADD    M \t\t\t ; adds the value of the memory location pointed by HL register to the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x87 => println!("ADD    A \t\t\t ; adds the value of A(cumulator) register to the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x88 => println!("ADC    B \t\t\t ; adds the value of B register to the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x89 => println!("ADC    C \t\t\t ; adds the value of C register to the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x8a => println!("ADC    D \t\t\t ; adds the value of D register to the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x8b => println!("ADC    E \t\t\t ; adds the value of E register to the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x8c => println!("ADC    H \t\t\t ; adds the value of H register to the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x8d => println!("ADC    L \t\t\t ; adds the value of L register to the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x8e => println!("ADC    M \t\t\t ; adds the value of the memory location pointed by HL register to the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x8f => println!("ADC    A \t\t\t ; adds the value of A(cumulator) register to the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x90 => println!("SUB    B \t\t\t ; subtracts the value of B register from the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x91 => println!("SUB    C \t\t\t ; subtracts the value of C register from the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x92 => println!("SUB    D \t\t\t ; subtracts the value of D register from the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x93 => println!("SUB    E \t\t\t ; subtracts the value of E register from the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x94 => println!("SUB    H \t\t\t ; subtracts the value of H register from the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x95 => println!("SUB    L \t\t\t ; subtracts the value of L register from the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x96 => println!("SUB    M \t\t\t ; subtracts the value of the memory location pointed by HL register from the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x97 => println!("SUB    A \t\t\t ; subtracts the value of A(cumulator) register from the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x98 => println!("SBB    B \t\t\t ; subtracts the value of B register from the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x99 => println!("SBB    C \t\t\t ; subtracts the value of C register from the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x9a => println!("SBB    D \t\t\t ; subtracts the value of D register from the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x9b => println!("SBB    E \t\t\t ; subtracts the value of E register from the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x9c => println!("SBB    H \t\t\t ; subtracts the value of H register from the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x9d => println!("SBB    L \t\t\t ; subtracts the value of L register from the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x9e => println!("SBB    M \t\t\t ; subtracts the value of the memory location pointed by HL register from the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0x9f => println!("SBB    A \t\t\t ; subtracts the value of A(cumulator) register from the value of A(cumulator) register and stores the result in A(cumulator) register"),
        0xa0 => println!("ANA    B \t\t\t ; performs a logical AND operation between the value of A register and the value of B register and stores the result in A register"),
        0xa1 => println!("ANA    C \t\t\t ; performs a logical AND operation between the value of A register and the value of C register and stores the result in A register"),
        0xa2 => println!("ANA    D \t\t\t ; performs a logical AND operation between the value of A register and the value of D register and stores the result in A register"),
        0xa3 => println!("ANA    E \t\t\t ; performs a logical AND operation between the value of A register and the value of E register and stores the result in A register"),
        0xa4 => println!("ANA    H \t\t\t ; performs a logical AND operation between the value of A register and the value of H register and stores the result in A register"),
        0xa5 => println!("ANA    L \t\t\t ; performs a logical AND operation between the value of A register and the value of L register and stores the result in A register"),
        0xa6 => println!("ANA    M \t\t\t ; performs a logical AND operation between the value of A register and the value of the memory location pointed by HL register and stores the result in A register"),
        0xa7 => println!("ANA    A \t\t\t ; performs a logical AND operation between the value of A register and the value of A register and stores the result in A register"),
        0xa8 => println!("XRA    B \t\t\t ; performs a logical XOR operation between the value of A register and the value of B register and stores the result in A register"),
        0xa9 => println!("XRA    C \t\t\t ; performs a logical XOR operation between the value of A register and the value of C register and stores the result in A register"),
        0xaa => println!("XRA    D \t\t\t ; performs a logical XOR operation between the value of A register and the value of D register and stores the result in A register"),
        0xab => println!("XRA    E \t\t\t ; performs a logical XOR operation between the value of A register and the value of E register and stores the result in A register"),
        0xac => println!("XRA    H \t\t\t ; performs a logical XOR operation between the value of A register and the value of H register and stores the result in A register"),
        0xad => println!("XRA    L \t\t\t ; performs a logical XOR operation between the value of A register and the value of L register and stores the result in A register"),
        0xae => println!("XRA    M \t\t\t ; performs a logical XOR operation between the value of A register and the value of the memory location pointed by HL register and stores the result in A register"),
        0xaf => println!("XRA    A \t\t\t ; performs a logical XOR operation between the value of A register and the value of A register and stores the result in A register"),
        0xb0 => println!("ORA    B \t\t\t ; performs a logical OR operation between the value of A register and the value of B register and stores the result in A register"),
        0xb1 => println!("ORA    C \t\t\t ; performs a logical OR operation between the value of A register and the value of C register and stores the result in A register"),
        0xb2 => println!("ORA    D \t\t\t ; performs a logical OR operation between the value of A register and the value of D register and stores the result in A register"),
        0xb3 => println!("ORA    E \t\t\t ; performs a logical OR operation between the value of A register and the value of E register and stores the result in A register"),
        0xb4 => println!("ORA    H \t\t\t ; performs a logical OR operation between the value of A register and the value of H register and stores the result in A register"),
        0xb5 => println!("ORA    L \t\t\t ; performs a logical OR operation between the value of A register and the value of L register and stores the result in A register"),
        0xb6 => println!("ORA    M \t\t\t ; performs a logical OR operation between the value of A register and the value of the memory location pointed by HL register and stores the result in A register"),
        0xb7 => println!("ORA    A \t\t\t ; performs a logical OR operation between the value of A register and the value of A register and stores the result in A register"),
        0xb8 => println!("CMP    B \t\t\t ; compares the value of A register with the value of B register and sets the Z flag to 1 if they are equal, otherwise sets it to 0"),
        0xb9 => println!("CMP    C \t\t\t ; compares the value of A register with the value of C register and sets the Z flag to 1 if they are equal, otherwise sets it to 0"),
        0xba => println!("CMP    D \t\t\t ; compares the value of A register with the value of D register and sets the Z flag to 1 if they are equal, otherwise sets it to 0"),
        0xbb => println!("CMP    E \t\t\t ; compares the value of A register with the value of E register and sets the Z flag to 1 if they are equal, otherwise sets it to 0"),
        0xbc => println!("CMP    H \t\t\t ; compares the value of A register with the value of H register and sets the Z flag to 1 if they are equal, otherwise sets it to 0"),
        0xbd => println!("CMP    L \t\t\t ; compares the value of A register with the value of L register and sets the Z flag to 1 if they are equal, otherwise sets it to 0"),
        0xbe => println!("CMP    M \t\t\t ; compares the value of A register with the value of the memory location pointed by HL register and sets the Z flag to 1 if they are equal, otherwise sets it to 0"),
        0xbf => println!("CMP    A \t\t\t ; compares the value of A register with the value of A register and sets the Z flag to 1 if they are equal, otherwise sets it to 0"),
        0xc0 => println!("RNZ \t\t\t ; if Z flag is not set, returns from subroutine"),
        0xc1 => println!("POP    B \t\t\t ; B and C registers are popped from the stack and SP (stack pointer) is incremented by 2"),
        0xc2 => {
            println!("JNZ    ${:02x}{:02x} \t\t ; if Z flag is not set, sets the PC (programm counter) pointer to Adress ${0:02x}{1:02x}", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0xc3 => {
            println!("JMP    ${:02x}{:02x} \t\t ; sets the PC (programm counter) pointer to Adress ${0:02x}{1:02x}", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0xc4 => {
            println!("CNZ    ${:02x}{:02x} \t\t ; if Z flag is not set, sets the PC (programm counter) pointer to Adress ${0:02x}{1:02x}", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0xc5 => println!("PUSH   B \t\t\t ; B and C registers are pushed to the stack and SP (stack pointer) is decremented by 2"),
        0xc6 => {
            println!("ADI    #${:02x} \t\t ; adds the value #${0:02x} to the A(cumulator) register", code_buffer[pc + 1]);
            opbytes = 2;
        }
        0xc7 => println!("RST    0 \t\t\t ; calls the subroutine at address 0"),
        0xc8 => println!("RZ \t\t\t ; if Z flag is set, returns from subroutine"),
        0xc9 => println!("RET \t\t\t ; returns from subroutine"), // TODO make this more specific
        0xca => {
            println!("JZ     ${:02x}{:02x} \t\t ; if Z flag is set, sets the PC (programm counter) pointer to Adress ${0:02x}{1:02x}", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0xcb => println!("NOP \t\t\t ; no operation /* (reserved) */"),
        0xcc => {
            println!("CZ     ${:02x}{:02x} \t\t ; if Z flag is set, calls the subroutine at address ${0:02x}{1:02x}", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0xcd => {
            println!("CALL   ${:02x}{:02x} \t\t ; calls the subroutine at address ${0:02x}{1:02x}", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        // ! empty
        0xd0 => println!("RNC \t\t\t ; if CY flag is not set, returns from subroutine"),
        0xd1 => println!("POP    D \t\t\t ; D and E registers are popped from the stack and SP (stack pointer) is incremented by 2"),
        0xd2 => {
            println!("JNC    ${:02x}{:02x} \t\t ; if CY flag is not set, sets the PC (programm counter) pointer to Adress ${0:02x}{1:02x}", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0xd3 => {
            println!("OUT    #${:02x} \t\t ; special", code_buffer[pc + 1]);
            opbytes = 2;
        }
        0xd4 => {
            println!("CNC    ${:02x}{:02x} \t\t ; if CY flag is not set, calls the subroutine at address ${0:02x}{1:02x}", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0xd5 => println!("PUSH   D \t\t\t ; D and E registers are pushed to the stack and SP (stack pointer) is decremented by 2"),
        0xd6 => {
            println!("SUI    #${:02x} \t\t ; subtracts the value #${0:02x} from the A(cumulator) register", code_buffer[pc + 1]);
            opbytes = 2;
        }
        0xd7 => println!("RST    2 \t\t\t ; calls the subroutine at address $10"),
        0xd8 => println!("RC \t\t\t ; if CY flag is set, returns from subroutine"),
        0xd9 => println!("NOP \t\t\t ; no operation /* (reserved) */"),
        0xda => {
            println!("JC     ${:02x}{:02x} \t\t ; if CY flag is set, sets the PC (programm counter) pointer to Adress ${0:02x}{1:02x}", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0xdb => {
            println!("IN     #${:02x} \t\t ; special", code_buffer[pc + 1]);
            opbytes = 2;
        }
        0xdc => {
            println!("CC     ${:02x}{:02x} \t\t ; if CY flag is set, calls the subroutine at address ${0:02x}{1:02x}", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0xdd => println!("NOP \t\t\t ; no operation /* (reserved) */"),
        0xde => {
            println!("SBI    #${:02x} \t\t ; subtracts the value #${0:02x} from the A(cumulator) register", code_buffer[pc + 1]);
            opbytes = 2;
        }
        0xdf => println!("RST    3 \t\t\t ; calls the subroutine at address $18"),
        0xe0 => println!("RPO \t\t\t ; if P flag is not set, returns from subroutine"),
        0xe1 => println!("POP    H \t\t\t ; H and L registers are popped from the stack and SP (stack pointer) is incremented by 2"),
        0xe2 => {
            println!("JPO    ${:02x}{:02x} \t\t ; if P flag is not set, sets the PC (programm counter) pointer to Adress ${0:02x}{1:02x}", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0xe3 => println!("XTHL \t\t\t ; exchanges the values of HL and SP registers"),
        0xe4 => {
            println!("CPO    ${:02x}{:02x} \t\t ; if P flag is not set, calls the subroutine at address ${0:02x}{1:02x}", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0xe5 => println!("PUSH   H \t\t\t ; H and L registers are pushed to the stack and SP (stack pointer) is decremented by 2"),
        0xe6 => {
            println!("ANI    #${:02x} \t\t ; performs a logical AND operation between the value of A register and the value #${0:02x} and stores the result in A register", code_buffer[pc + 1]);
            opbytes = 2;
        }
        0xe7 => println!("RST    4 \t\t\t ; calls the subroutine at address $20"),
        0xe8 => println!("RPE \t\t\t ; if P flag is set, returns from subroutine"),
        0xe9 => println!("PCHL \t\t\t ; sets the PC (programm counter) pointer to the value of HL register"),
        0xea => {
            println!("JPE    ${:02x}{:02x} \t\t ; if P flag is set, sets the PC (programm counter) pointer to Adress ${0:02x}{1:02x}", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0xeb => println!("XCHG \t\t\t ; exchanges the values of HL and DE registers"),
        0xec => {
            println!("CPE    ${:02x}{:02x} \t\t ; if P flag is set, calls the subroutine at address ${0:02x}{1:02x}", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0xed => println!("NOP \t\t\t ; no operation /* (reserved) */"),
        0xee => {
            println!("XRI    #${:02x} \t\t ; performs a logical XOR operation between the value of A register and the value #${0:02x} and stores the result in A register", code_buffer[pc + 1]);
            opbytes = 2;
        }
        0xef => println!("RST    5 \t\t\t ; calls the subroutine at address $28"),
        0xf0 => println!("RP \t\t\t ; if S flag is not set, returns from subroutine"),
        0xf1 => println!("POP    PSW \t\t ; flags and A(cumulator) register are popped from the stack and SP (stack pointer) is incremented by 2"),
        0xf2 => {
            println!("JP     ${:02x}{:02x} \t\t ; if P flag is set, sets the PC (programm counter) pointer to Adress ${0:02x}{1:02x}", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0xf3 => println!("DI \t\t\t ; special"),
        0xf4 => {
            println!("CP     ${:02x}{:02x} \t\t ; if P flag is set, calls the subroutine at address ${0:02x}{1:02x}", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0xf5 => println!("PUSH   PSW \t\t ; flags and A(cumulator) register are pushed to the stack and SP (stack pointer) is decremented by 2"),
        0xf6 => {
            println!("ORI    #${:02x} \t\t ; performs a logical OR operation between the value of A register and the value #${0:02x} and stores the result in A register", code_buffer[pc + 1]);
            opbytes = 2;
        }
        0xf7 => println!("RST    6 \t\t\t ; calls the subroutine at address $30"),
        0xf8 => println!("RM \t\t\t ; if S flag is set, returns from subroutine"),
        0xf9 => println!("SPHL \t\t\t ; sets the value of HL register to SP register"),
        0xfa => {
            println!("JM     ${:02x}{:02x} \t\t ; if S flag is set, sets the PC (programm counter) pointer to Adress ${0:02x}{1:02x}", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0xfb => println!("EI \t\t\t ; special"),
        0xfc => {
            println!("CM     ${:02x}{:02x} \t\t ; if S flag is set, calls the subroutine at address ${0:02x}{1:02x}", code_buffer[pc + 2], code_buffer[pc + 1]);
            opbytes = 3;
        }
        0xfd => println!("NOP \t\t\t ; no operation /* (reserved) */"),
        0xfe => {
            println!("CPI    #${:02x} \t\t ; compares the value of A register with the value #${0:02x}", code_buffer[pc + 1]);
            opbytes = 2;
        }
        0xff => println!("RST    7 \t\t\t ; calls the subroutine at address $38"),
        _ => {
            println!("Unknown opcode: {:02x}", *code);
        }
    }

    opbytes
}
