use super::Instruction;
use super::RawInstruction;
use std::fmt;

/// # Decoder
///
/// Decodes and executes instructions
pub struct Decoder;

const MAIN_OPCODE_MASK: RawInstruction = 0xF000;

impl Decoder {
    /// Decodes a 2-byte instruction into a struct with decoded operands
    pub fn decode(raw_instruction: RawInstruction) -> Instruction {
        // This one is pretty common
        if raw_instruction == 0 {return Instruction::Nop};
        let opcode = raw_instruction & MAIN_OPCODE_MASK;  // 4 most sig. bits
        match opcode {
            0x0000 | 0x1000 | 0x2000 => {
                let rd = ((raw_instruction & 0x01F0) >> 4) as u8;
                let mut rr = (raw_instruction & 0x000F) as u8;
                if raw_instruction & 0x0200 != 0 {rr += 16}
                Instruction::TwoRegOp{op: raw_instruction >> 10, rd, rr}
                },
            0x4000 | 0x5000 | 0x6000 | 0x7000 | 0xE000 => {
                let rd = ((raw_instruction & 0x00F0) >> 4) as u8;
                let constant_upper = ((raw_instruction & 0x0F00) >> 4) as u8;
                let constant_lower = (raw_instruction & 0x000F) as u8;
                let constant = constant_upper + constant_lower;
                Instruction::RegConstOp{op: raw_instruction >> 12, rd, constant}
            },
            0x8000 | 0xA000 => { // LDD / STD
                let is_load = raw_instruction & 0x0200 == 0;
                let is_base_z = raw_instruction & 0x0008 == 0;
                let reg = ((raw_instruction & 0x01F0) >> 4) as u8;
                let (offset_lo, offset_mid, offset_hi) = (raw_instruction & 0x7,
                    (raw_instruction & 0x0C00) >> 7, (raw_instruction & 0x2000) >> 8);
                let offset =  offset_lo + offset_mid + offset_hi;
                Instruction::TransferIndirect{is_load, is_base_z, reg, offset: offset as u8}
            },
            0x9000 => { // One register operations?
                match raw_instruction & 0x0E00 {
                    0 | 0x0200 => {
                        let is_pop = raw_instruction & 0x0200 == 0;
                        let reg = ((raw_instruction & 0x01F0) >> 4) as u8;
                        if raw_instruction & 0x000F == 0xF {
                            Instruction::PushPop {is_pop, reg}
                        } else {
                            Instruction::OneRegOp
                        }
                    }
                    _ => Instruction::OneRegOp
                }
            },
            0xB000 => {
                let is_in = raw_instruction & 0x0800 == 0;
                let reg = ((raw_instruction & 0x01F0) >> 4) as u8;
                let address_low = (raw_instruction & 0x000F) as u8;
                let address_hi = ((raw_instruction & 0x0600) >> 5) as u8;
                let address = address_hi + address_low;
                Instruction::InOut{is_in, reg, address}
            },
            0xC000 | 0xD000 => {
                let is_call = opcode == 0xD000;
                let offset = raw_instruction & 0xFFF;
                Instruction::CallJmp{is_call, relative: true, address: offset}
            },
            _ => {
                warn!("Decoding - Unknown opcode: {:x} in {:x}", opcode, raw_instruction);
                Instruction::Unsupported {instruction: raw_instruction}
            }
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Nop => write!(f, "nop"),
            Instruction::TwoRegOp { op, rd, rr } => 
                write!(f, "Two register operation -> op: {} rd: {}, rr:{}",
                   *op, *rd, *rr),
            Instruction::RegConstOp {op, rd, constant } => 
                write!(f, "Operation against constant -> op: {} rd: {}, constant:{}",
                   *op, *rd, *constant),
            Instruction::PushPop { is_pop, reg } => {
                let op_str = if *is_pop { "pop" } else { "push" };
                write!(f, "{} r{}", op_str, *reg)
            },
            Instruction::TransferIndirect { is_load, is_base_z, reg, offset } =>  {
                let op_str = if *is_load { "ldd" } else { "std" };
                let base_reg = if *is_base_z { "Z" } else { "Y" };
                write!(f, "{} {}+{}, r{}", op_str, base_reg, offset, reg)
            },
            Instruction::CallJmp { is_call, relative, address } => {
                let r_str = if *relative { "r" } else { "" };
                let op_str = if *is_call { "call" } else { "jmp" };
                write!(f, "{}{}, ${}", r_str, op_str, *address)
            },
            Instruction::InOut {is_in, reg, address } => {
                let op_str = if *is_in { "in" } else { "out" };
                write!(f, "{} r{} ${:x}", op_str, *reg, *address)
            },
            Instruction::OneRegOp => write!(f, "Parsed but unsupported instruction"),
            Instruction::Unsupported { instruction } => 
                write!(f, "Unsupported instruction: {:x}", *instruction)
        }
    }
}
