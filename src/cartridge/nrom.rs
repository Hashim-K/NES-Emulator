
struct nrom {
    charactor_memory_size: usize,
    character_memory_writable: bool,
    character_memory: [u8],

    program_rom_size: usize,
    program_rom: [u8],

    mirroring: bool,
    peristent_memory: bool,
    ignore_mirroring_control: bool,

}
