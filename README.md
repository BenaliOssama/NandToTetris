# From NAND to Tetris — Part I

> Building a modern computer from first principles.

## What is nand2tetris?

Most people who write code never see what happens below the programming language. nand2tetris is a course that tears away every layer of abstraction — from logic gates all the way up to a working computer — and makes you build each one yourself.

The starting point is a single logic gate: **NAND**. Every other gate (AND, OR, NOT, XOR...) can be built from NAND alone. From those gates you build chips. From those chips you build a CPU. From that CPU you run programs you wrote in assembly. By the end you have a complete, working computer called **Hack**, built entirely from scratch.

This repository is my implementation of **Part I** of the course, which covers the hardware platform and the assembler. Part II (compiler + operating system) is coming next.

---

## Certificate

I completed Part I and received the certificate from the course.

---

## What I Built — Project by Project

### Project 1 — Boolean Logic (gates)
Implemented all fundamental logic gates in HDL (Hardware Description Language), starting from the primitive NAND gate:
`Not`, `And`, `Or`, `Xor`, `Mux`, `DMux`, and their 16-bit / multi-way variants (`And16`, `Or16`, `Mux16`, `Mux4Way16`, `Mux8Way16`, `DMux4Way`, `DMux8Way`).

### Project 2 — Boolean Arithmetic (ALU)
Built the arithmetic layer on top of the gates:
- `HalfAdder`, `FullAdder`, `Add16`, `Inc16`
- **ALU** — the Arithmetic Logic Unit, the heart of the CPU. It takes two 16-bit inputs and a set of control bits, and can compute addition, subtraction, bitwise AND/OR, negation, and zero in a single chip.

### Project 3 — Sequential Logic (memory)
Introduced the clock and state. Built memory chips that remember values across time:
- `Bit` (1-bit register), `Register` (16-bit), `RAM8`, `RAM64`, `RAM512`, `RAM4K`, `RAM16K`
- **PC** — the Program Counter, which tracks which instruction the CPU executes next.

### Project 4 — Machine Language (assembly programming)
Wrote programs directly in Hack assembly to understand the instruction set:
- `mult.asm` — multiplies two numbers using repeated addition
- `fill.asm` — reads the keyboard and fills/clears the screen in real time (every pixel, written by hand in assembly)

### Project 5 — Computer Architecture (the full Hack computer)
Connected everything into a complete computer:
- `Memory.hdl` — unified address space: RAM, screen buffer, and keyboard register
- `CPU.hdl` — the full Hack CPU: A/D registers, ALU, program counter, and all jump logic wired together
- `Computer.hdl` — top-level chip: CPU + Memory + ROM, a complete stored-program computer

### Project 6 — Assembler (written in Rust)
Wrote a full assembler from scratch in Rust that translates Hack assembly (`.asm`) into binary machine code (`.hack`):
- Two-pass parser: first pass collects all labels into a symbol table, second pass translates instructions
- Handles A-instructions (`@value`, `@symbol`), C-instructions (`dest=comp;jump`), labels `(LOOP)`, and all predefined symbols
- Strips comments (line and block) and whitespace before processing
- Successfully assembles the official Pong game (`Pong.asm` → `Pong.hack`)

```
assembler/
├── src/
│   ├── main.rs         # CLI entry point
│   ├── lib.rs          # Top-level pipeline: clean → process → write
│   └── processing.rs   # Symbol table + two-pass translation
```

**Usage:**
```bash
cd assembler
cargo run -- path/to/program.asm
# outputs: output.hack
```

---

## Repository Structure

```
nand2tetris/
├── project1/      # Logic gates (HDL)
├── project2/      # ALU and adders (HDL)
├── project3/      # Memory chips (HDL)
├── project4/      # Assembly programs (.asm)
├── project5/      # Full Hack computer (HDL)
├── assembler/     # Hack assembler written in Rust
└── project7/      # VM translator — early work in progress (Rust)
```

The `projects/` folder contains the original course materials provided by the instructors (test scripts, comparison files, and example programs).

---

## What's Next — Part II

Part I ends with a working hardware platform and assembler. Part II builds the software stack on top:

- **Project 7 & 8** — VM Translator: translates a stack-based intermediate language into Hack assembly
- **Project 9** — High-level language: writing programs in Jack (a simple OOP language)
- **Project 10 & 11** — Jack Compiler: tokenizer + full compiler from Jack down to VM code
- **Project 12** — Operating System: standard library written in Jack (memory, math, I/O, strings)

The goal is a complete toolchain: Jack source code → compiler → VM code → VM translator → assembly → assembler → binary running on the Hack computer built in Part I.

---

## Tech Stack

| Layer | Technology |
|---|---|
| Hardware description | HDL (nand2tetris) |
| Assembly programs | Hack assembly |
| Assembler | Rust |
| VM translator (WIP) | Rust |
