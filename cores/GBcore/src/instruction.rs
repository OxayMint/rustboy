use core::fmt;

// pub use AddressMode;
// pub use RegisterType;
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub address_mode: AddressMode,
    pub register_1: RegisterType,
    pub register_2: RegisterType,
    pub condition: ConditionType,
    pub opcode: u8,
    pub rst_vec: u8,
    pub length: u8,           // length of the instruction
    pub cycles: u8, //cycles count for NONE condition operations and for conditions that had taken action
    pub no_action_cycles: u8, //if condition is not NONE this should be set. it is the cycles count of the operation if the condition fails
}
#[derive(PartialEq, Debug)]

pub enum AddressMode {
    IMPLIED,
    R_D16,
    R_R,
    MR_R,
    R,
    R_D8,
    R_MR,
    R_HLI,
    R_HLD,
    HLI_R,
    HLD_R,
    R_A8,
    A8_R,
    HL_SPR,
    D16,
    D8,
    D16_R,
    MR_D8,
    MR,
    A16_R,
    R_A16,
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
pub enum RegisterType {
    N,
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(PartialEq, Debug)]
pub enum InstructionType {
    NONE,
    NOP,
    LD,
    INC,
    DEC,
    RLCA,
    ADD,
    RRCA,
    STOP,
    RLA,
    JR,
    RRA,
    DAA,
    CPL,
    SCF,
    CCF,
    HALT,
    ADC,
    SUB,
    SBC,
    AND,
    XOR,
    OR,
    CP,
    POP,
    JP,
    PUSH,
    RET,
    CB,
    CALL,
    RETI,
    LDH,
    // JPHL,
    DI,
    EI,
    RST,
    ERR,
    //CB instructions...
    // RLC,
    // RRC,
    // RL,
    // RR,
    // SLA,
    // SRA,
    // SWAP,
    // SRL,
    // BIT,
    // RES,
    // SET,
}
impl RegisterType {
    pub fn decode(reg: usize) -> RegisterType {
        if reg > 0b111 {
            RegisterType::N
        } else {
            [
                RegisterType::B,
                RegisterType::C,
                RegisterType::D,
                RegisterType::E,
                RegisterType::H,
                RegisterType::L,
                RegisterType::HL,
                RegisterType::A,
            ][reg]
                .clone()
        }
    }
}
impl fmt::Display for ConditionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl fmt::Display for RegisterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for AddressMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}",)
    }
}
impl fmt::Display for InstructionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(PartialEq, Debug)]
pub enum ConditionType {
    NONE,
    NZ,
    Z,
    NC,
    C,
}

impl Instruction {
    pub fn from_opcode(code: &u8) -> Self {
        let mut inst = match code {
            0x00 => Instruction {
                instruction_type: InstructionType::NOP,
                ..Default::default()
            },
            0xE8 => Instruction {
                instruction_type: InstructionType::ADD,
                address_mode: AddressMode::R_D8,
                register_1: RegisterType::SP,
                ..Default::default()
            },
            0x01 => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::R_D16,
                register_1: RegisterType::BC,
                ..Default::default()
            },
            0x02 => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::MR_R,
                register_1: RegisterType::BC,
                register_2: RegisterType::A,
                ..Default::default()
            },
            0x03 => Instruction {
                instruction_type: InstructionType::INC,
                address_mode: AddressMode::R,
                register_1: RegisterType::BC,
                ..Default::default()
            },
            0x04 => Instruction {
                instruction_type: InstructionType::INC,
                address_mode: AddressMode::R,
                register_1: RegisterType::B,
                ..Default::default()
            },
            0x05 => Instruction {
                instruction_type: InstructionType::DEC,
                address_mode: AddressMode::R,
                register_1: RegisterType::B,
                ..Default::default()
            },
            0x06 => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::R_D8,
                register_1: RegisterType::B,
                ..Default::default()
            },
            0x07 => Instruction {
                instruction_type: InstructionType::RLCA,
                ..Default::default()
            },
            0x08 => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::A16_R,
                register_1: RegisterType::N,
                register_2: RegisterType::SP,
                ..Default::default()
            },
            0x09 => Instruction {
                instruction_type: InstructionType::ADD,
                address_mode: AddressMode::R_R,
                register_1: RegisterType::HL,
                register_2: RegisterType::BC,
                ..Default::default()
            },
            0x0A => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::R_MR,
                register_1: RegisterType::A,
                register_2: RegisterType::BC,
                ..Default::default()
            },
            0x0B => Instruction {
                instruction_type: InstructionType::DEC,
                address_mode: AddressMode::R,
                register_1: RegisterType::BC,
                ..Default::default()
            },
            0x0C => Instruction {
                instruction_type: InstructionType::INC,
                address_mode: AddressMode::R,
                register_1: RegisterType::C,
                ..Default::default()
            },
            0x0D => Instruction {
                instruction_type: InstructionType::DEC,
                address_mode: AddressMode::R,
                register_1: RegisterType::C,
                ..Default::default()
            },
            0x0E => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::R_D8,
                register_1: RegisterType::C,
                ..Default::default()
            },
            0x0F => Instruction {
                instruction_type: InstructionType::RRCA,
                ..Default::default()
            },
            0x10 => Instruction {
                instruction_type: InstructionType::STOP,
                address_mode: AddressMode::D8,
                ..Default::default()
            },
            0x11 => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::R_D16,
                register_1: RegisterType::DE,
                ..Default::default()
            },
            0x12 => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::MR_R,
                register_1: RegisterType::DE,
                register_2: RegisterType::A,
                ..Default::default()
            },
            0x13 => Instruction {
                instruction_type: InstructionType::INC,
                address_mode: AddressMode::R,
                register_1: RegisterType::DE,
                ..Default::default()
            },
            0x14 => Instruction {
                instruction_type: InstructionType::INC,
                address_mode: AddressMode::R,
                register_1: RegisterType::D,
                ..Default::default()
            },
            0x15 => Instruction {
                instruction_type: InstructionType::DEC,
                address_mode: AddressMode::R,
                register_1: RegisterType::D,
                ..Default::default()
            },
            0x16 => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::R_D8,
                register_1: RegisterType::D,
                ..Default::default()
            },
            0x17 => Instruction {
                instruction_type: InstructionType::RLA,
                ..Default::default()
            },
            0x18 => Instruction {
                instruction_type: InstructionType::JR,
                address_mode: AddressMode::D8,
                ..Default::default()
            },
            0x19 => Instruction {
                instruction_type: InstructionType::ADD,
                address_mode: AddressMode::R_R,
                register_1: RegisterType::HL,
                register_2: RegisterType::DE,
                ..Default::default()
            },
            0x1A => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::R_MR,
                register_1: RegisterType::A,
                register_2: RegisterType::DE,
                ..Default::default()
            },
            0x1B => Instruction {
                instruction_type: InstructionType::DEC,
                address_mode: AddressMode::R,
                register_1: RegisterType::DE,
                ..Default::default()
            },
            0x1C => Instruction {
                instruction_type: InstructionType::INC,
                address_mode: AddressMode::R,
                register_1: RegisterType::E,
                ..Default::default()
            },
            0x1D => Instruction {
                instruction_type: InstructionType::DEC,
                address_mode: AddressMode::R,
                register_1: RegisterType::E,
                ..Default::default()
            },
            0x1E => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::R_D8,
                register_1: RegisterType::E,
                ..Default::default()
            },
            0x1F => Instruction {
                instruction_type: InstructionType::RRA,
                ..Default::default()
            },
            0x20 => Instruction {
                instruction_type: InstructionType::JR,
                address_mode: AddressMode::D8,
                condition: ConditionType::NZ,
                ..Default::default()
            },
            0x21 => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::R_D16,
                register_1: RegisterType::HL,
                ..Default::default()
            },
            0x22 => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::HLI_R,
                register_1: RegisterType::HL,
                register_2: RegisterType::A,
                ..Default::default()
            },
            0x23 => Instruction {
                instruction_type: InstructionType::INC,
                address_mode: AddressMode::R,
                register_1: RegisterType::HL,
                ..Default::default()
            },
            0x24 => Instruction {
                instruction_type: InstructionType::INC,
                address_mode: AddressMode::R,
                register_1: RegisterType::H,
                ..Default::default()
            },
            0x25 => Instruction {
                instruction_type: InstructionType::DEC,
                address_mode: AddressMode::R,
                register_1: RegisterType::H,
                ..Default::default()
            },
            0x26 => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::R_D8,
                register_1: RegisterType::H,
                ..Default::default()
            },
            0x27 => Instruction {
                instruction_type: InstructionType::DAA,
                ..Default::default()
            },
            0x28 => Instruction {
                instruction_type: InstructionType::JR,
                address_mode: AddressMode::D8,
                condition: ConditionType::Z,
                ..Default::default()
            },
            0x29 => Instruction {
                instruction_type: InstructionType::ADD,
                address_mode: AddressMode::R_R,
                register_1: RegisterType::HL,
                register_2: RegisterType::HL,
                ..Default::default()
            },
            0x2A => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::R_HLI,
                register_1: RegisterType::A,
                register_2: RegisterType::HL,
                ..Default::default()
            },
            0x2B => Instruction {
                instruction_type: InstructionType::DEC,
                address_mode: AddressMode::R,
                register_1: RegisterType::HL,
                ..Default::default()
            },
            0x2C => Instruction {
                instruction_type: InstructionType::INC,
                address_mode: AddressMode::R,
                register_1: RegisterType::L,
                ..Default::default()
            },
            0x2D => Instruction {
                instruction_type: InstructionType::DEC,
                address_mode: AddressMode::R,
                register_1: RegisterType::L,
                ..Default::default()
            },
            0x2E => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::R_D8,
                register_1: RegisterType::L,
                ..Default::default()
            },
            0x2F => Instruction {
                instruction_type: InstructionType::CPL,
                ..Default::default()
            },
            0x30 => Instruction {
                instruction_type: InstructionType::JR,
                address_mode: AddressMode::D8,
                condition: ConditionType::NC,
                ..Default::default()
            },
            0x31 => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::R_D16,
                register_1: RegisterType::SP,
                ..Default::default()
            },
            0x32 => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::HLD_R,
                register_1: RegisterType::HL,
                register_2: RegisterType::A,
                ..Default::default()
            },
            0x33 => Instruction {
                instruction_type: InstructionType::INC,
                address_mode: AddressMode::R,
                register_1: RegisterType::SP,
                ..Default::default()
            },
            0x34 => Instruction {
                instruction_type: InstructionType::INC,
                address_mode: AddressMode::MR,
                register_1: RegisterType::HL,
                ..Default::default()
            },
            0x35 => Instruction {
                instruction_type: InstructionType::DEC,
                address_mode: AddressMode::MR,
                register_1: RegisterType::HL,
                ..Default::default()
            },
            0x36 => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::MR_D8,
                register_1: RegisterType::HL,
                ..Default::default()
            },
            0x37 => Instruction {
                instruction_type: InstructionType::SCF,
                ..Default::default()
            },
            0x38 => Instruction {
                instruction_type: InstructionType::JR,
                address_mode: AddressMode::D8,
                condition: ConditionType::C,
                ..Default::default()
            },
            0x39 => Instruction {
                instruction_type: InstructionType::ADD,
                address_mode: AddressMode::R_R,
                register_1: RegisterType::HL,
                register_2: RegisterType::SP,
                ..Default::default()
            },
            0x3A => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::R_HLD,
                register_1: RegisterType::A,
                register_2: RegisterType::HL,
                ..Default::default()
            },
            0x3B => Instruction {
                instruction_type: InstructionType::DEC,
                address_mode: AddressMode::R,
                register_1: RegisterType::SP,
                ..Default::default()
            },
            0x3C => Instruction {
                instruction_type: InstructionType::INC,
                address_mode: AddressMode::R,
                register_1: RegisterType::A,
                ..Default::default()
            },
            0x3D => Instruction {
                instruction_type: InstructionType::DEC,
                address_mode: AddressMode::R,
                register_1: RegisterType::A,
                ..Default::default()
            },
            0x3E => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::R_D8,
                register_1: RegisterType::A,
                ..Default::default()
            },
            0x3F => Instruction {
                instruction_type: InstructionType::CCF,
                ..Default::default()
            },
            0x76 => {
                return Instruction {
                    instruction_type: InstructionType::HALT,

                    ..Default::default()
                };
            }
            0x40..=0x7F => {
                let regs = [
                    RegisterType::B,  //4  with r:off                                    0
                    RegisterType::C,  //4 with r:on  1
                    RegisterType::D,  //5  with r:off                                    2
                    RegisterType::E,  //5 with r:on  3
                    RegisterType::H,  //6  with r:off                                    4
                    RegisterType::L,  //6 with r:on  5
                    RegisterType::HL, //7 with r:off                                     6
                    RegisterType::A,  //7 with r:on  7
                ];

                let left_digit = code >> 4;
                let right_digit = code & 0xf;
                let is_right_side = (right_digit >= 8) as u8;

                let reg_1_idx = (left_digit - 4) * 2 + is_right_side;
                let reg_2_idx = right_digit - (8 * is_right_side);
                let reg1 = &regs[reg_1_idx as usize];
                let reg2 = &regs[reg_2_idx as usize];
                let addr_mode = if *reg1 == RegisterType::HL {
                    AddressMode::MR_R
                } else if *reg2 == RegisterType::HL {
                    AddressMode::R_MR
                } else {
                    AddressMode::R_R
                };
                Instruction {
                    instruction_type: InstructionType::LD,
                    address_mode: addr_mode,
                    register_1: reg1.clone(),
                    register_2: reg2.clone(),
                    ..Default::default()
                }
            }
            0x80..=0x87 => Instruction {
                instruction_type: InstructionType::ADD,
                address_mode: if code & 0x07 == 0x06 {
                    AddressMode::R_MR
                } else {
                    AddressMode::R_R
                },
                register_1: RegisterType::A,
                register_2: match code & 0x07 {
                    0 => RegisterType::B,
                    1 => RegisterType::C,
                    2 => RegisterType::D,
                    3 => RegisterType::E,
                    4 => RegisterType::H,
                    5 => RegisterType::L,
                    6 => RegisterType::HL,
                    7 => RegisterType::A,
                    _ => unreachable!(),
                },
                ..Default::default()
            },
            0x88..=0x8F => Instruction {
                instruction_type: InstructionType::ADC,
                address_mode: if code & 0x07 == 0x06 {
                    AddressMode::R_MR
                } else {
                    AddressMode::R_R
                },
                register_1: RegisterType::A,
                register_2: match code & 0x07 {
                    0 => RegisterType::B,
                    1 => RegisterType::C,
                    2 => RegisterType::D,
                    3 => RegisterType::E,
                    4 => RegisterType::H,
                    5 => RegisterType::L,
                    6 => RegisterType::HL,
                    7 => RegisterType::A,
                    _ => unreachable!(),
                },
                ..Default::default()
            },
            0x90..=0x97 => Instruction {
                instruction_type: InstructionType::SUB,
                address_mode: if code & 0x07 == 0x06 {
                    AddressMode::R_MR
                } else {
                    AddressMode::R_R
                },
                register_1: RegisterType::A,
                register_2: match code & 0x07 {
                    0 => RegisterType::B,
                    1 => RegisterType::C,
                    2 => RegisterType::D,
                    3 => RegisterType::E,
                    4 => RegisterType::H,
                    5 => RegisterType::L,
                    6 => RegisterType::HL,
                    7 => RegisterType::A,
                    _ => unreachable!(),
                },
                ..Default::default()
            },
            0x98..=0x9F => Instruction {
                instruction_type: InstructionType::SBC,
                address_mode: if code & 0x07 == 0x06 {
                    AddressMode::R_MR
                } else {
                    AddressMode::R_R
                },
                register_1: RegisterType::A,
                register_2: match code & 0x07 {
                    0 => RegisterType::B,
                    1 => RegisterType::C,
                    2 => RegisterType::D,
                    3 => RegisterType::E,
                    4 => RegisterType::H,
                    5 => RegisterType::L,
                    6 => RegisterType::HL,
                    7 => RegisterType::A,
                    _ => unreachable!(),
                },
                ..Default::default()
            },
            0xA0..=0xA7 => Instruction {
                instruction_type: InstructionType::AND,
                address_mode: if code & 0x07 == 0x06 {
                    AddressMode::R_MR
                } else {
                    AddressMode::R_R
                },
                register_1: RegisterType::A,
                register_2: match code & 0x07 {
                    0 => RegisterType::B,
                    1 => RegisterType::C,
                    2 => RegisterType::D,
                    3 => RegisterType::E,
                    4 => RegisterType::H,
                    5 => RegisterType::L,
                    6 => RegisterType::HL,
                    7 => RegisterType::A,
                    _ => unreachable!(),
                },
                ..Default::default()
            },
            0xA8..=0xAF => Instruction {
                instruction_type: InstructionType::XOR,
                address_mode: if (code & 0x07) == 0x06 {
                    AddressMode::R_MR
                } else {
                    AddressMode::R_R
                },
                register_1: RegisterType::A,
                register_2: match code & 0x07 {
                    0 => RegisterType::B,
                    1 => RegisterType::C,
                    2 => RegisterType::D,
                    3 => RegisterType::E,
                    4 => RegisterType::H,
                    5 => RegisterType::L,
                    6 => RegisterType::HL,
                    7 => RegisterType::A,
                    _ => unreachable!(),
                },
                ..Default::default()
            },
            0xB0..=0xB7 => Instruction {
                instruction_type: InstructionType::OR,
                address_mode: if code & 0x07 == 0x06 {
                    AddressMode::R_MR
                } else {
                    AddressMode::R_R
                },
                register_1: RegisterType::A,
                register_2: match code & 0x07 {
                    0 => RegisterType::B,
                    1 => RegisterType::C,
                    2 => RegisterType::D,
                    3 => RegisterType::E,
                    4 => RegisterType::H,
                    5 => RegisterType::L,
                    6 => RegisterType::HL,
                    7 => RegisterType::A,
                    _ => unreachable!(),
                },
                ..Default::default()
            },
            0xB8..=0xBF => Instruction {
                instruction_type: InstructionType::CP,
                address_mode: if code & 0x07 == 0x06 {
                    AddressMode::R_MR
                } else {
                    AddressMode::R_R
                },
                register_1: RegisterType::A,
                register_2: match code & 0x07 {
                    0 => RegisterType::B,
                    1 => RegisterType::C,
                    2 => RegisterType::D,
                    3 => RegisterType::E,
                    4 => RegisterType::H,
                    5 => RegisterType::L,
                    6 => RegisterType::HL,
                    7 => RegisterType::A,
                    _ => unreachable!(),
                },
                ..Default::default()
            },
            0xC0 => Instruction {
                instruction_type: InstructionType::RET,

                condition: ConditionType::NZ,
                ..Default::default()
            },
            0xC1 => Instruction {
                instruction_type: InstructionType::POP,
                address_mode: AddressMode::R,
                register_1: RegisterType::BC,
                ..Default::default()
            },
            0xC2 => Instruction {
                instruction_type: InstructionType::JP,
                address_mode: AddressMode::D16,
                condition: ConditionType::NZ,
                ..Default::default()
            },
            0xC3 => Instruction {
                instruction_type: InstructionType::JP,
                address_mode: AddressMode::D16,
                ..Default::default()
            },
            0xC4 => Instruction {
                instruction_type: InstructionType::CALL,
                address_mode: AddressMode::D16,
                condition: ConditionType::NZ,
                ..Default::default()
            },
            0xC5 => Instruction {
                instruction_type: InstructionType::PUSH,
                address_mode: AddressMode::R,
                register_1: RegisterType::BC,
                ..Default::default()
            },
            0xC6 => Instruction {
                instruction_type: InstructionType::ADD,
                address_mode: AddressMode::R_D8,
                register_1: RegisterType::A,
                ..Default::default()
            },
            0xC7 => Instruction {
                instruction_type: InstructionType::RST,
                rst_vec: 0x00,
                ..Default::default()
            },
            0xC8 => Instruction {
                instruction_type: InstructionType::RET,
                condition: ConditionType::Z,
                ..Default::default()
            },
            0xC9 => Instruction {
                instruction_type: InstructionType::RET,
                ..Default::default()
            },
            0xCA => Instruction {
                instruction_type: InstructionType::JP,
                address_mode: AddressMode::D16,
                condition: ConditionType::Z,
                ..Default::default()
            },
            0xCB => Instruction {
                instruction_type: InstructionType::CB,
                address_mode: AddressMode::D8,
                ..Default::default()
            },

            0xCC => Instruction {
                instruction_type: InstructionType::CALL,
                address_mode: AddressMode::D16,
                condition: ConditionType::Z,
                ..Default::default()
            },
            0xCD => Instruction {
                instruction_type: InstructionType::CALL,
                address_mode: AddressMode::D16,
                ..Default::default()
            },
            0xCE => Instruction {
                instruction_type: InstructionType::ADC,
                address_mode: AddressMode::R_D8,
                register_1: RegisterType::A,
                ..Default::default()
            },
            0xCF => Instruction {
                instruction_type: InstructionType::RST,
                rst_vec: 0x08,
                ..Default::default()
            },
            0xD0 => Instruction {
                instruction_type: InstructionType::RET,
                condition: ConditionType::NC,
                ..Default::default()
            },
            0xD1 => Instruction {
                instruction_type: InstructionType::POP,
                address_mode: AddressMode::R,
                register_1: RegisterType::DE,
                ..Default::default()
            },
            0xD2 => Instruction {
                instruction_type: InstructionType::JP,
                address_mode: AddressMode::D16,
                condition: ConditionType::NC,
                ..Default::default()
            },
            0xD3 => Instruction {
                instruction_type: InstructionType::NONE,
                ..Default::default()
            },
            0xD4 => Instruction {
                instruction_type: InstructionType::CALL,
                address_mode: AddressMode::D16,
                condition: ConditionType::NC,
                ..Default::default()
            },
            0xD5 => Instruction {
                instruction_type: InstructionType::PUSH,
                address_mode: AddressMode::R,
                register_1: RegisterType::DE,
                ..Default::default()
            },
            0xD6 => Instruction {
                instruction_type: InstructionType::SUB,
                address_mode: AddressMode::R_D8,
                register_1: RegisterType::A,
                ..Default::default()
            },
            0xD7 => Instruction {
                instruction_type: InstructionType::RST,
                rst_vec: 0x10,
                ..Default::default()
            },
            0xD8 => Instruction {
                instruction_type: InstructionType::RET,
                condition: ConditionType::C,
                ..Default::default()
            },
            0xD9 => Instruction {
                instruction_type: InstructionType::RETI,
                ..Default::default()
            },
            0xDA => Instruction {
                instruction_type: InstructionType::JP,
                address_mode: AddressMode::D16,
                condition: ConditionType::C,
                ..Default::default()
            },
            0xDB => Instruction {
                instruction_type: InstructionType::NONE,
                ..Default::default()
            },
            0xDC => Instruction {
                instruction_type: InstructionType::CALL,
                address_mode: AddressMode::D16,
                condition: ConditionType::C,
                ..Default::default()
            },
            0xDD => Instruction {
                instruction_type: InstructionType::NONE,
                ..Default::default()
            },
            0xDE => Instruction {
                instruction_type: InstructionType::SBC,
                address_mode: AddressMode::R_D8,
                register_1: RegisterType::A,
                ..Default::default()
            },
            0xDF => Instruction {
                instruction_type: InstructionType::RST,
                rst_vec: 0x18,
                ..Default::default()
            },
            0xE0 => Instruction {
                instruction_type: InstructionType::LDH,
                address_mode: AddressMode::A8_R,
                register_1: RegisterType::N,
                register_2: RegisterType::A,
                ..Default::default()
            },
            0xE1 => Instruction {
                instruction_type: InstructionType::POP,
                address_mode: AddressMode::R,
                register_1: RegisterType::HL,
                ..Default::default()
            },
            0xE2 => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::MR_R,
                register_1: RegisterType::C,
                register_2: RegisterType::A,
                ..Default::default()
            },
            0xE3 => Instruction {
                instruction_type: InstructionType::NONE,
                ..Default::default()
            },
            0xE4 => Instruction {
                instruction_type: InstructionType::NONE,
                ..Default::default()
            },
            0xE5 => Instruction {
                instruction_type: InstructionType::PUSH,
                address_mode: AddressMode::R,
                register_1: RegisterType::HL,
                ..Default::default()
            },
            0xE6 => Instruction {
                instruction_type: InstructionType::AND,
                address_mode: AddressMode::R_D8,
                register_1: RegisterType::A,
                ..Default::default()
            },
            0xE7 => Instruction {
                instruction_type: InstructionType::RST,
                rst_vec: 0x20,
                ..Default::default()
            },
            0xE9 => Instruction {
                instruction_type: InstructionType::JP,
                address_mode: AddressMode::R,
                register_1: RegisterType::HL,
                ..Default::default()
            },
            0xEA => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::A16_R,
                register_1: RegisterType::N,
                register_2: RegisterType::A,
                ..Default::default()
            },
            0xEB => Instruction {
                instruction_type: InstructionType::NONE,
                ..Default::default()
            },
            0xEC => Instruction {
                instruction_type: InstructionType::NONE,
                ..Default::default()
            },
            0xED => Instruction {
                instruction_type: InstructionType::NONE,
                ..Default::default()
            },
            0xEE => Instruction {
                instruction_type: InstructionType::XOR,
                address_mode: AddressMode::R_D8,
                register_1: RegisterType::A,
                ..Default::default()
            },
            0xEF => Instruction {
                instruction_type: InstructionType::RST,
                rst_vec: 0x28,
                ..Default::default()
            },
            0xF0 => Instruction {
                instruction_type: InstructionType::LDH,
                address_mode: AddressMode::R_A8,
                register_1: RegisterType::A,
                ..Default::default()
            },
            0xF1 => Instruction {
                instruction_type: InstructionType::POP,
                address_mode: AddressMode::R,
                register_1: RegisterType::AF,
                ..Default::default()
            },
            0xF2 => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::R_MR,
                register_1: RegisterType::A,
                register_2: RegisterType::C,
                ..Default::default()
            },
            0xF3 => Instruction {
                instruction_type: InstructionType::DI,
                ..Default::default()
            },
            0xF4 => Instruction {
                instruction_type: InstructionType::NONE,
                ..Default::default()
            },
            0xF5 => Instruction {
                instruction_type: InstructionType::PUSH,
                address_mode: AddressMode::R,
                register_1: RegisterType::AF,
                ..Default::default()
            },
            0xF6 => Instruction {
                instruction_type: InstructionType::OR,
                address_mode: AddressMode::R_D8,
                register_1: RegisterType::A,
                ..Default::default()
            },
            0xF7 => Instruction {
                instruction_type: InstructionType::RST,
                rst_vec: 0x30,
                ..Default::default()
            },
            0xF8 => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::HL_SPR,
                register_1: RegisterType::HL,
                register_2: RegisterType::SP,
                ..Default::default()
            },
            0xF9 => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::R_R,
                register_1: RegisterType::SP,
                register_2: RegisterType::HL,
                ..Default::default()
            },
            0xFA => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::R_A16,
                register_1: RegisterType::A,
                ..Default::default()
            },
            0xFB => Instruction {
                instruction_type: InstructionType::EI,
                ..Default::default()
            },
            0xFC => Instruction {
                instruction_type: InstructionType::NONE,
                ..Default::default()
            },
            0xFD => Instruction {
                instruction_type: InstructionType::NONE,
                ..Default::default()
            },
            0xFE => Instruction {
                instruction_type: InstructionType::CP,
                address_mode: AddressMode::R_D8,
                register_1: RegisterType::A,
                ..Default::default()
            },
            0xFF => Instruction {
                instruction_type: InstructionType::RST,
                rst_vec: 0x38,
                ..Default::default()
            },
        };
        // inst.length = Instruction::length(&inst);
        inst.opcode = *code;
        return inst;
    }
    // Instruction {}
    // pub fn start_opcodetests() {
    //     for n in 0x00..=0xff {
    //         println!("{:x}: {}", n, Instruction::from_opcode(&n).to_string());
    //     }
    // }
    pub fn to_string(&self) -> String {
        // let addr1 = match self.address_mode {
        //     AddressMode::R_D16 | AddressMode::R_R | AddressMode::R => todo!(),
        //     AddressMode::R_D8
        //     | AddressMode::R_MR
        //     | AddressMode::R_HLI
        //     | AddressMode::R_HLD
        //     | AddressMode::R_A8
        //     | AddressMode::R_A16 => self.register_1,

        //     AddressMode::MR_R |
        //     AddressMode::MR_D8 |
        //     AddressMode::MR => self
        //     AddressMode::HLI_R => todo!(),
        //     AddressMode::HLD_R => todo!(),
        //     AddressMode::A8_R => todo!(),
        //     AddressMode::HL_SPR => todo!(),
        //     AddressMode::D16 => todo!(),
        //     AddressMode::D8 => todo!(),
        //     AddressMode::D16_R => todo!(),
        //     AddressMode::A16_R => todo!(),
        // };
        return format!(
            "{: <5} {: <8} {: <2} {: <2}  {} ", //{: >2}/{: >2}",
            self.instruction_type.to_string(),
            self.address_mode.to_string(),
            self.register_1.to_string(),
            self.register_2.to_string(),
            self.length.to_string(),
            // self.cycles.to_string(),
            // self.no_action_cycles.to_string()
        );
    }
    /* pub fn length(inst: &Instruction) -> u8 {
         match inst.address_mode {
             AddressMode::IMPLIED => 1,
             AddressMode::R | AddressMode::MR => 1,
             AddressMode::R_R | AddressMode::R_MR | AddressMode::MR_R => 1,
             AddressMode::R_D8
             | AddressMode::MR_D8
             | AddressMode::A8_R
             | AddressMode::R_A8
             | AddressMode::HL_SPR
             | AddressMode::D8
             | AddressMode::R_HLI
             | AddressMode::R_HLD
             | AddressMode::HLI_R
             | AddressMode::HLD_R => 2,
             AddressMode::R_D16
             | AddressMode::R_A16
             | AddressMode::D16_R
             | AddressMode::A16_R
             | AddressMode::D16 => 3,
         }
     }
    */
    // pub fn get_cycles_count(inst: &Instruction) -> (u8, Option<u8>) {
    //     match (&inst.instruction_type, &inst.address_mode, &inst.condition) {
    //         // Conditional instructions
    //         (InstructionType::JR, _, ConditionType::NONE) => (12, None),
    //         (InstructionType::JR, _, _) => (12, Some(8)),
    //         (InstructionType::JP, AddressMode::D16, ConditionType::NONE) => (16, None),
    //         (InstructionType::JP, AddressMode::D16, _) => (16, Some(12)),
    //         (InstructionType::CALL, _, ConditionType::NONE) => (24, None),
    //         (InstructionType::CALL, _, _) => (24, Some(12)),
    //         (InstructionType::RET, _, ConditionType::NONE) => (16, None),
    //         (InstructionType::RET, _, _) => (20, Some(8)),

    //         // Non-conditional instructions
    //         (InstructionType::NOP, _, _) => (4, None),
    //         (InstructionType::LD, AddressMode::R_R, _) => (4, None),
    //         (InstructionType::LD, AddressMode::R_D16, _) => (12, None),
    //         (InstructionType::LD, AddressMode::R_D8, _) => (8, None),
    //         (InstructionType::LD, AddressMode::R_MR, _) => (8, None),
    //         (InstructionType::LD, AddressMode::MR_R, _) => (8, None),
    //         (InstructionType::LD, AddressMode::R_HLI, _)
    //         | (InstructionType::LD, AddressMode::R_HLD, _) => (8, None),
    //         (InstructionType::LD, AddressMode::HLI_R, _)
    //         | (InstructionType::LD, AddressMode::HLD_R, _) => (8, None),
    //         (InstructionType::LD, AddressMode::A16_R, _) => (16, None),
    //         (InstructionType::LD, AddressMode::R_A16, _) => (16, None),
    //         (InstructionType::LD, AddressMode::HL_SPR, _) => (12, None),
    //         (InstructionType::INC, AddressMode::R, _)
    //         | (InstructionType::DEC, AddressMode::R, _) => (4, None),
    //         (InstructionType::INC, AddressMode::MR, _)
    //         | (InstructionType::DEC, AddressMode::MR, _) => (12, None),
    //         (InstructionType::RLCA, _, _)
    //         | (InstructionType::RRCA, _, _)
    //         | (InstructionType::RLA, _, _)
    //         | (InstructionType::RRA, _, _) => (4, None),
    //         (InstructionType::ADD, AddressMode::R_R, _) if inst.register_1 == RegisterType::HL => {
    //             (8, None)
    //         }
    //         (InstructionType::ADD, AddressMode::R_R, _) => (4, None),
    //         (InstructionType::ADD, AddressMode::R_D8, _) => (8, None),
    //         (InstructionType::DAA, _, _)
    //         | (InstructionType::CPL, _, _)
    //         | (InstructionType::SCF, _, _)
    //         | (InstructionType::CCF, _, _) => (4, None),
    //         (InstructionType::HALT, _, _) => (4, None),
    //         (InstructionType::ADC, AddressMode::R_R, _)
    //         | (InstructionType::SBC, AddressMode::R_R, _) => (4, None),
    //         (InstructionType::ADC, AddressMode::R_D8, _)
    //         | (InstructionType::SBC, AddressMode::R_D8, _) => (8, None),
    //         (InstructionType::AND, AddressMode::R_R, _)
    //         | (InstructionType::XOR, AddressMode::R_R, _)
    //         | (InstructionType::OR, AddressMode::R_R, _)
    //         | (InstructionType::CP, AddressMode::R_R, _) => (4, None),
    //         (InstructionType::AND, AddressMode::R_D8, _)
    //         | (InstructionType::XOR, AddressMode::R_D8, _)
    //         | (InstructionType::OR, AddressMode::R_D8, _)
    //         | (InstructionType::CP, AddressMode::R_D8, _) => (8, None),
    //         (InstructionType::POP, _, _) => (12, None),
    //         (InstructionType::PUSH, _, _) => (16, None),
    //         (InstructionType::JP, AddressMode::R, _) => (4, None),
    //         (InstructionType::RST, _, _) => (16, None),

    //         // (InstructionType::PREFIX, _, _) => (4, None),
    //         (InstructionType::RETI, _, _) => (16, None),
    //         (InstructionType::LDH, _, _) => (12, None),
    //         (InstructionType::DI, _, _) | (InstructionType::EI, _, _) => (4, None),
    //         (InstructionType::STOP, _, _) => (4, None),
    //         (InstructionType::SUB, AddressMode::R_R, _)
    //         | (InstructionType::SUB, AddressMode::R_MR, _) => (4, None),
    //         (InstructionType::SUB, AddressMode::R_D8, _) => (8, None),

    //         // Default case
    //         _ => (4, None),
    //     }
    // }
}

impl Default for Instruction {
    fn default() -> Instruction {
        Instruction {
            instruction_type: InstructionType::NONE,
            address_mode: AddressMode::IMPLIED,
            register_1: RegisterType::A,
            register_2: RegisterType::B,
            condition: ConditionType::NONE,
            rst_vec: 0x00,
            length: 1,
            cycles: 4,
            no_action_cycles: 4,
            opcode: 0,
        }
    }
}
