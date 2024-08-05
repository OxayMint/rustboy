pub struct Instruction {
    pub instruction_type: InstructionType,
    pub address_mode: AddressMode,
    pub register_1: RegisterType,
    pub register_2: RegisterType,
    pub condition: ConditionType,
    pub param: u8,
}

impl Instruction {
    pub fn from_opcode(code: u8) -> Self {
        match code {
            0x00 => Instruction {
                instruction_type: InstructionType::NOP,
                ..Default::default()
            },
            0x05 => Instruction {
                instruction_type: InstructionType::DEC,
                address_mode: AddressMode::R,
                register_1: RegisterType::B,
                ..Default::default()
            },
            0xc3 => Instruction {
                instruction_type: InstructionType::JP,
                address_mode: AddressMode::D16,
                ..Default::default()
            },
            0xaf => Instruction {
                instruction_type: InstructionType::XOR,
                address_mode: AddressMode::R_R,
                register_1: RegisterType::C,
                ..Default::default()
            },
            0x0e => Instruction {
                instruction_type: InstructionType::LD,
                address_mode: AddressMode::R_D8,
                register_1: RegisterType::C,
                ..Default::default()
            },
            1..=u8::MAX => Instruction {
                instruction_type: InstructionType::NONE,
                ..Default::default()
            },
        }
        // Instruction {}
    }
}

impl Default for Instruction {
    fn default() -> Instruction {
        Instruction {
            instruction_type: InstructionType::NONE,
            address_mode: AddressMode::IMPLIED,
            register_1: RegisterType::A,
            register_2: RegisterType::B,
            condition: ConditionType::CT_NONE,
            param: 0,
        }
    }
}

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

pub enum RegisterType {
    NONE,
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
    JPHL,
    DI,
    EI,
    RST,
    ERR,
    //CB instructions...
    RLC,
    RRC,
    RL,
    RR,
    SLA,
    SRA,
    SWAP,
    SRL,
    BIT,
    RES,
    SET,
}

pub enum ConditionType {
    CT_NONE,
    CT_NZ,
    CT_Z,
    CT_NC,
    CT_C,
}
