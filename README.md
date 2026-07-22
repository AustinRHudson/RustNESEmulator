# Rust NES Emulator

A Nintendo Entertainment System emulator written entirely in Rust, built from the ground up to explore low-level systems programming, CPU emulation, graphics rendering, memory mapping, and hardware behavior.

The project recreates the major components of the original NES hardware and executes NES ROMs through custom implementations of the 6502 CPU, PPU, memory bus, cartridge system, input handling, and audio processing.

## Features

* **6502 CPU emulation**

  * Instruction decoding and execution
  * Addressing modes
  * CPU registers and status flags
  * Stack operations
  * Interrupt handling
  * Cycle-aware execution

* **PPU graphics emulation**

  * Background rendering
  * Nametable and pattern table handling
  * Sprite rendering
  * Scrolling
  * Palette handling
  * NES framebuffer generation

* **Cartridge and memory system**

  * iNES ROM loading
  * PRG-ROM and CHR-ROM handling
  * NES memory mapping
  * CPU and PPU address spaces

* **APU audio emulation**

  * Pulse channels
  * Triangle channel
  * Envelope processing
  * Length counters
  * Sweep units
  * NES-style nonlinear audio mixing

## Screenshots

<p align="center">
  <img src="https://github.com/user-attachments/assets/32e9516c-2022-43a8-b7e7-158476e64406" width="70%" alt="Super Mario Bros running in the emulator">
  <img src="https://github.com/user-attachments/assets/b2f5702b-4ea1-4000-b38a-1db8ca28c0ea" width="70%" alt="Pac-Man running in the emulator">
  <img src="https://github.com/user-attachments/assets/f437c7d3-014b-4bb9-930c-a8800353be38" width="70%" alt="Donkey Kong running in the emulator">
</p>

## Architecture

The emulator is organized around the major hardware components of the NES:

```text
                 ┌────────────────────┐
                 │        CPU         │
                 │      6502 CPU       │
                 └─────────┬──────────┘
                           │
                           ▼
                 ┌────────────────────┐
                 │        BUS         │
                 │  Memory / Devices  │
                 └──────┬───────┬─────┘
                        │       │
              ┌─────────▼─┐   ┌─▼─────────┐
              │    PPU    │   │    APU    │
              │ Graphics  │   │   Audio   │
              └───────────┘   └───────────┘
                        │
                        ▼
                 ┌────────────────────┐
                 │     Cartridge      │
                 │  PRG-ROM / CHR-ROM │
                 └────────────────────┘
```

The CPU, PPU, and APU operate independently while communicating through the emulated NES bus and memory-mapped registers.

## Getting Started

### Prerequisites

You will need:

* Rust and Cargo
* SDL2 development libraries

The emulator uses SDL2 for rendering, input, and audio.

### Clone the repository

```bash
git clone https://github.com/AustinRHudson/RustNESEmulator.git
cd RustNESEmulator
```

### Run the emulator

```bash
cargo run --release
```

To run a ROM, provide a compatible `.nes` file according to the emulator's current ROM-loading configuration.

## Testing

The project includes CPU testing and debugging resources used during development.

For example, the repository includes execution logs and resources for validating CPU behavior against established NES test ROMs.

Run the Rust test suite with:

```bash
cargo test
```

## Development Goals

This project is primarily an exploration of how hardware emulation works at a low level.

Some of the major goals include:

* Accurately emulate the 6502 CPU
* Reproduce NES PPU timing and rendering behavior
* Implement cycle-accurate hardware interactions
* Improve APU timing and sound quality
* Expand cartridge mapper support
* Increase compatibility with commercial NES games
* Improve automated testing and debugging tools

## Resources

This project was developed using a combination of technical documentation, hardware references, and test ROMs.

* [NESdev Wiki](https://www.nesdev.org/wiki/)
* [6502 CPU Reference](https://www.nesdev.org/obelisk-6502-guide/reference.html)
* [NES Test ROMs](https://github.com/christopherpow/nes-test-roms)

## Why I Built This

I built this emulator to get a more hands-on experience with low-level/embedded systems and work with stuff below the typical level of traditional application development.

Rather than relying on existing emulation libraries, the major hardware components are implemented from scratch in Rust. The project has been an opportunity to work with CPU architecture, memory buses, hardware timing, graphics pipelines, digital audio, debugging, and systems-level programming.

## License

This project is intended for educational purposes. Please do not distribute copyrighted ROM files.

---

Built from scratch in Rust.

