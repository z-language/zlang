#[repr(u8)]
pub enum Opcode {
    NOOP = 0x00,

    PUSH = 0x01,
    LOAD_CONST = 0x02,
    STORE_NAME = 0x03,
    LOAD_NAME = 0x04,
    POP = 0x12,
    DUP = 0x13,

    ADD = 0x20,
    SUB = 0x21,
    MUL = 0x22,
    DIV = 0x23,
    MOD = 0x24,
    EQ = 0x25,
    GREATER_THAN = 0x26,
    GREATER_OR_EQ = 0x27,
    LESS_THAN = 0x28,
    LESS_OR_EQ = 0x29,
    AND = 0x30,
    OR = 0x31,

    JMP = 0x40,
    JMPF = 0x41,
    JMPB = 0x42,
    JMPT = 0x43,

    CALL = 0x50,
    RETURN = 0x51,

    SYS = 0xfd,
    DEBUG = 0xfe,
    HLT = 0xff,
}

pub enum Type {
    T_STR = 0x01,
    T_INT = 0x02,
}
