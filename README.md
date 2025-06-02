# NES Emulator Project

This project is a Nintendo Entertainment System (NES) emulator developed as part of the CESE4000: Software Fundamentals course at TU Delft. The goal was to recreate the core architecture of the NES, focusing on emulating the 6502 Central Processing Unit (CPU) and its interactions with memory, cartridges, and the Picture Processing Unit (PPU).

---

## Key Features & Technical Details

### NES Core Architecture Emulation
The project successfully emulates the fundamental components of the NES.

### CPU Emulation (6502)
* The emulator replicates the functionality of the 6502 CPU, a processor known for its Complex Instruction Set Computing (CISC) architecture
* It handles instruction decoding, execution, and timing, including operations that load data from memory, manipulate it, and store it back in a single instruction.
* Addressing modes are implemented to determine how instructions access memory, with exceptions for implied addressing and write-only instructions to prevent unintended behavior with registers like PPUDATA.
* The system manages instruction cycle timing, adding additional cycles for events like page crossing and branch success.
* Non-maskable Interrupts (NMI) are implemented, with the CPU polling for and handling these interrupts. Interrupt hijacking for BRK operations is also supported.

### Memory Emulation
* A comprehensive memory system manages the NES's address space, redirecting reads and writes to the correct components. This includes internal memory, PPU, cartridge, and controller.
* **Cartridge Emulation**: The `Cartridge` struct is responsible for reading ROM files, interpreting headers, and handling memory mapping (NROM and MMC1 mappers are implemented). It manages Program ROM (prg data), Character ROM (chr data), PRG RAM, and CHR RAM. Bank switching for both PRG and CHR ROM is supported.
* **Controller Emulation**: The `Controller` struct handles reads and writes for a standard NES controller.

### System Architecture
* The emulator is structured with different program crates representing the physically separate parts of the NES (CPU, PPU, APU, cartridge, controller).
* A top-level struct implements `TestableCPU` and `CPU` traits, containing the memory struct that maps addresses to system components.

### Debug and Testing
* CPU instruction logging is implemented in a format comparable to other emulators, facilitating instruction-by-instruction comparison for error detection.
* The `Log` crate is used for debugging, integrating with PPU logging. Log output includes relevant CPU state for each instruction (e.g., `C009 AD 02 20 LDA A:00 X:FF Y:00 P:26 SP:FF CYC:201`).

### Continuous Integration
* Gitlab's Continuous Integration was utilized to automatically test if the code compiles, runs tests successfully, adheres to correct formatting, and passes Clippy's linting tests.

---

## Development Process & Collaboration

This project emphasized collaborative software development. The team maintained consistent weekly hours, engaged in frequent and productive discussions, and utilized Gitlab issues and pull requests for development and planning. While some unforeseen bugs caused delays, the project highlights experience in tackling complex system emulation and collaborative problem-solving.
