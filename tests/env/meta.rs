const ISA_TESTS: &[(&str, &str)] = &[
    ("rv32ui", "add"),
    ("rv32ui", "addi"),
    ("rv32ui", "and"),
    ("rv32ui", "andi"),
    ("rv32ui", "auipc"),
    ("rv32ui", "beq"),
    ("rv32ui", "bge"),
    ("rv32ui", "bgeu"),
    ("rv32ui", "blt"),
    ("rv32ui", "bltu"),
    ("rv32ui", "bne"),
    ("rv32ui", "fence_i"),
    ("rv32ui", "jal"),
    ("rv32ui", "jalr"),
    ("rv32ui", "lb"),
    ("rv32ui", "lbu"),
    ("rv32ui", "lh"),
    ("rv32ui", "lhu"),
    ("rv32ui", "lui"),
    ("rv32ui", "lw"),
    ("rv32ui", "or"),
    ("rv32ui", "ori"),
    ("rv32ui", "sb"),
    ("rv32ui", "sh"),
    ("rv32ui", "simple"),
    ("rv32ui", "sll"),
    ("rv32ui", "slli"),
    ("rv32ui", "slt"),
    ("rv32ui", "slti"),
    ("rv32ui", "sltiu"),
    ("rv32ui", "sltu"),
    ("rv32ui", "sra"),
    ("rv32ui", "srai"),
    ("rv32ui", "srl"),
    ("rv32ui", "srli"),
    ("rv32ui", "sub"),
    ("rv32ui", "sw"),
    ("rv32ui", "xor"),
    ("rv32ui", "xori"),

    ("rv32um", "div"),
    ("rv32um", "divu"),
    ("rv32um", "mul"),
    ("rv32um", "mulh"),
    ("rv32um", "mulhsu"),
    ("rv32um", "mulhu"),
    ("rv32um", "rem"),
    ("rv32um", "remu"),

    ("rv32ua", "amoadd_w"),
    ("rv32ua", "amoand_w"),
    ("rv32ua", "amomax_w"),
    ("rv32ua", "amomaxu_w"),
    ("rv32ua", "amomin_w"),
    ("rv32ua", "amominu_w"),
    ("rv32ua", "amoor_w"),
    ("rv32ua", "amoswap_w"),
    ("rv32ua", "amoxor_w"),
    ("rv32ua", "lrsc"),

    ("rv32uf", "fadd"),
    ("rv32uf", "fclass"),
    ("rv32uf", "fcmp"),
    ("rv32uf", "fcvt"),
    ("rv32uf", "fcvt_w"),
    ("rv32uf", "fdiv"),
    ("rv32uf", "fmadd"),
    ("rv32uf", "fmin"),
    ("rv32uf", "ldst"),
    ("rv32uf", "move"),
    ("rv32uf", "recoding"),

    ("rv32ud", "fadd"),
    ("rv32ud", "fclass"),
    ("rv32ud", "fcmp"),
    ("rv32ud", "fcvt"),
    ("rv32ud", "fcvt_w"),
    ("rv32ud", "fdiv"),
    ("rv32ud", "fmadd"),
    ("rv32ud", "fmin"),
    ("rv32ud", "ldst"),
    // ("rv32ud", "move"),
    ("rv32ud", "recoding"),
];
