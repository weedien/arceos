macro_rules! include_asm_marcos {
    () => {
        #[cfg(target_arch = "riscv32")]
        global_asm!(r"
        .macro REG_L rd, rs, offset
            lw \rd, \offset(\rs)
        .endmacro

        .macro REG_S rs2, rs1, offset
            sw \rs2, \offset(\rs1)
        .endmacro
        ");

        #[cfg(target_arch = "riscv64")]
        global_asm!(r"
        .macro REG_L rd, rs, offset
            ld \rd, \offset(\rs)
        .endmacro

        .macro REG_S rs2, rs1, offset
            sd \rs2, \offset(\rs1)
        .endmacro
        ");
    }
}
