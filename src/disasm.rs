/// DISASM.RS
/// Provides disassembly of Z80 opcodes
/// Information provided from http://www.z80.info/decoding.htm
/// 
/// FORMAT:
///     Instructions are in the following 2 formats:
///         [] denote optional
///         () only used to group wording, no effect
///
///         [PREFIX BYTE] OPCODE [DISPLACEMENT BYTE] [IMMEDIATE DATA]
///         or
///         (TWO PREFIX BYTES) (DISPLACEMENT BYTE) OPCODE
///
/// BYTE VALUES:
///     A prefix byte has the values:
///         CB, DD, ED, or FD
///         and will always begin with one of those values.
///
///     Displacement byte is a i8 and is used to identify offset to add to a given memory address.
///
///     Opcode identifies the type of instruction to execute.
///     
///     Immediate data can be either 0, 1, or 2 bytes of additional information for the instruction 
///         and length will depend on the instruction.
///
/// OPCODE VALUES:
///
///     The opcode is divided up into different parts:
///         [7] [6] [5] [4] [3] [2] [1] [0]
///         ^-----^ ^---------^ ^---------^
///            x         y           z
///
///                 [5] [4] [3]
///                 ^-----^ ^-^
///                    p     q
///
///         x: bits 6-7
///         y: bits 3-5
///             p: bits 4-5
///             q: bit  3
///         z: bits 0-2
///
///     These values are used to decode the opcodes
pub mod disasm {
    struct Opcode {
        opcode: u8,
        x: u8,
        y: u8,
        z: u8,
        p: u8,
        q: u8
    }

    struct Instruction {
        prefix_a: u8,
        prefix_b: u8,
        opcode: Opcode,
        displacement: i8,
        immediate: u16
    }

    impl Opcode {
        fn new(op: u8) -> Opcode {
            Opcode {
                opcode: op,
                x: get_bits_inclusive(op, 6, 7),
                y: get_bits_inclusive(op, 3, 5),
                x: get_bits_inclusive(op, 0, 2),
                p: get_bits_inclusive(op, 4, 5),
                q: get_bits_inclusive(op, 3, 3)
            }
        }
    }

    impl Instruction {
        fn new(bytes: &[u8]) -> Instruction {
            let mut (prfx_a, prfx_b, displc, imm) = (0u8, 0u8, 0i8, 0u32);
            let mut opcode = Opcode::new(0);

            match bytes[0] {
                prfx @ 0xCB | prfx @ 0xDD | prfx @ 0xED | prfx @ 0xFD => {
                    
                },
                op => {
                    opcode = Opcode::new(op);
                    match opcode.x {
                        0 => match opcode.z {
                                0 => match opcode.y {
                                        4..7 => displc = bytes[1],
                                        _ => 
                                     },
                                1 => if opcode.q == 0 { imm = combine_u8s(bytes[1], bytes[2]); },
                                2 => if opcode.p == 2 || opcode.p == 3 { imm = combine_u8s(bytes[1], bytes[2]); },
                                6 => imm = bytes[1] as u16,
                                _ => 
                             },
                        3 => match opcode.z {
                                2 => imm = combine_u8s(bytes[1], bytes[2]),
                                3 => match opcode.y {
                                        0 => imm = combine_u8s(bytes[1], bytes[2]),
                                        2 | 3 => imm = bytes[1] as u16,
                                        _ =>
                                     },
                                4 => ,
                                5 => if opcode.p == 0 { imm = combine_u8s(bytes[1], bytes[2]); },
                                6 => imm = bytes[1] as u16,
                                _ =>
                             },
                        _ => 
                    }
                }
            }

            Instruction { prefix_a: prfx_a, prefix_b: prfx_b, opcode: opcode, displacement: dispc, immediate: imm }
        }

        fn to_mnemonic(&self) -> &str {

        }
    }

    fn combine_u8s(a: &u8, b: &u8) -> u16 {
        (a as u16 << 8) | b as u16
    }

    fn get_bits_inclusive(operand: &u8, low: u8, high: u8) -> u8 {
        (operand >> low) & ~(~0 << (high-low+1))
    }
}