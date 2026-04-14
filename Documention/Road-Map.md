## This is a roadmap of what needs to be done to create the Gameboy Emulator - Rust.

# Phase 0:

Stablizing the Core.

- Fix/Verify register and flag helpers (AF, BC, DE, HL)
- Individual flag set/get
- Add unit tests for:
    * Register pair split/join correctness
    * Each flag bit behavior
    * reset/inital CPU state
- Add a step(&mut self, mem: & mut Memory) -> u8 method that returns cycles
- Implement fetch_byte and fetch_word using PC.

Notes:

There are 8-bit registers:
    - A  (accumulator)
    - F  (flags)
    - B, C, D, E, H, L

It also uses some of them  as 16-bit pairs:
    -  AF
    -  BC
    -  DE
    -  HL

Register helpers are small methods that convert between: two 8-bet registers (B + C) and one 16-bit value (BC)

So these methods will convert B and C into one 16-bit value called BC. Combine returns a u16, split takes the u16 and writes high byte to B, low byte to C. 

Example of a split to two 8-bit registers:

We have a 16-bit value:
    value = 0x1234 (1, 2, 3, 4 are each 4 bits)
    binary: 0001 0010 0011 0100
For register pair BC:
    - high byte (0x12) goes to B
    - low byte (0x34) goes to C

In RUST:
    let value: u16 = 0x1234;
    let b: u8 = (value >> 8) as u8; // 0x12 , >> 8 moves high byte down into low position
    let c: u8 = (value & 0x00FF) as u8; // 0x34 , & 0x00FF keeps only low byte
    
Example for two 8-bit registers combined into one 16-bit value

We have two seperate values:
    - B = 0x12
    - C = 0x34

Combined into BC = 0x1234

let bc: u16 ((b as u16) << 8) | (c ad u16); //0x1234

Flag Helpers:

F is a special register containing status bits (flags), not normal data. 

Game Boy flags:
    * (zero) bit 7
    * (subtract) bit 6
    * (half-carry) bit 5
    * (carry) bit 4
So flag helpers are methods like:
    - get_flag_z() -> bool
    - set_flag_z(bool)

AF is A which is used for arithmetic/logical instructions.
      F records instruction results (zero/carry/etc).

BC, DE, HL:
    used as 16-bit counters/operands in some instructions.
    HL is heavily used as a memory pointer (HL Addressing)

So when instructions run, they often read/write these pairs, not just single registers.

Registers are storage inside the CPU.
    They hold:
        - numbers used by instructions
        - addresses (or parts of addresses)
        -  intermediate results
        - status infos (flags)

8 bit                                                           vs.         16 bit
* most arithmetic/logical instructions operate on 8-bit values              addresses (PC, SP), register paris(HL, BC, DE)
* memory is byte-addressed                                                  Some add/increment/jump behavior
* opcodes are byte-based                                            

Registers in action (Simple examples)
Example A: load and add
    Start: A = 0x10, B = 0x22
    Instruction: ADD A, B
    End: 0x32
A and B acted like CPU scratch values for math.

Example B:  register pair as address
    H = 0xC0, L = 0x10 => HL = 0xC010
    Instruction:  LD A, (HL) - Data fetched is 8-bit
    means: read memory at address 0xC010 into A
    So HL is commonly used as a pointer register pair.

Opcodes is numeric code for a CPU instruction. Emulator reads the opcode, 0x80 might mean ADD A, B, decodes this and runs matching behavior.

BIG TINY MENTAL SIMULATOR (ADD A, B + Flags)

Lets say we have:

A = 0x8F (1000_1111)
and 
B = 0x01 (0000_0001)

Math equals 1000_1111 + 0000_0001 = 1001_0000

Now we move to the flags
    - Z (zero): result is 0x90, not zero -> Z = 0
    - N (subtract): this was addition -> N = 0
    - H (half-carry): did low nibble overflow?
    low  nibble: 0xF + 0x1 = 0x10  overflow past 4 bits -> H = 1
    - C (carry):  did full 8-bit overflow past 0xFF - we dont lose the 9th bit its stored in the flag.
    0x8F +  0x01 is 0x90, no -> C = 0

Final:
    A =  0x90
    flags, Z = 0, N=0, H=1, C=0

# Phase 1:

Minimal CPU Execution

- Add opcode decode via match opcode.
- Implement a small starter set first:
    * NOP
    * LD r, n
    * LD, r1, r2
    * INC/DEC r
    * ADD A, r
    * JP nn
    * JR e
- Ensure every instruction updates PC, flags, and cycle counts correctly.
- Add tests per instruction family (arrange RAM + Registers, run one step, assert result).

# Phase 2:

Complete CPU Instruction Set

- Implement remaining non-CB opcodes
- Implement CB-prefixed bit/rotate/shift instructions.
- Add stack/control flow (CALL, RET, RST, PUSH/POP)
- Add interrupt-related CPU behavior (EI, DI, HALT, STOP) basic
- Run CPU-focused test ROMs and iterate until pass.

# Phase 3:

Proper Memory Bus + Cartridge

- Replace flat "just RAM" behavior with a mapped bus:
    * ROM area
    * VRAM
    * WRAM
    * HRAM
    * I/O registers
- Add cartridge loader (.gb) and header parsing
- Implement MBC1 first (enough for many games), then MBC3/MBC5 later.

# Phase 4:

Timing + Interrupts + Timer/Joypad

- Implement DIV/TIMA/TMA/TAC.
- Implement IF/IE interrupt flow and vector jumps.
- Implement joypad register behavior
- Validate timing-sensitive tests where possible.

# Phase 5:

PPU + Video Outputs (SDL2)

- Implement PPU modes and scanline progression.
- Render background/window/sprites to framebuffer
- Display framebuffer with SDL2
- Target: boot logo/ simple game visuals appear

# Phase 6:

Audio + Polish

- Basic APU channels (optional early, but needed for "full emulator" feel).
- Save RAM support.
- CLI options (ROM path, speed controls, debug logs)
- Improve performance and accuracy.

# Phase 7:

"Functioning Emulator" Definition:

It can be a function emulator when it passes the following conditions:
- It loads ROMs reliably
- It passes core CPU tests.
- It renders graphics correctly in common titles.
- Inputs work.
- At least one commercial/homebrew game is playable.