# Chip-8-Emulator
*A simple chip-8 emulator written by Rust*

*--UIUC CS128 Honor Final Project*

## Basic information
#### Group name: 
ILoveRust123
#### Group member: 
Yanze Lu : yanzelu2@illinois.edu

Jiayi Yan :  jiayi22@illinois.edu
#### Description: 
Chip-8 was a very simple programming language used for many of the 8-bit games from the 70s. This project is a simple implement of the chip-8 emulator. May include memory allocation, game loop, handling graphics. This is a very classic and charming project.
## Technical Overview: 
### Project structure:

The overall philosophy of our project is **five main tasks: IFDER**

*Input -> Fetch -> Decode -> Execute -> Render*

#### 0. Most critical classes
The Main Loop : An infinite game loop (aka.the driver class) setup the callback, call the chip-8 class member functions, finally update and clear the display.
CPU classes : Store all the variables and methods in CPU during the whole process.
#### 1. Input 
Utilities classes: keypad translator, (timer). 
[might be implemented] Rom-loading class : loads the contents of a file. 

#### 2. Fetch
In CPU class : Fetching the codes on 4-bits basis from the memory.
Random Number Generator : places a random number into a register.

#### 3. Decode and Execution
In CPU class : Using a considerable amount of `matching pattern`, decode and exec every command.

#### 4.Render
Fonts-loading class : convey the pixel character to the corresponding graphic images.
May use WASM to interact `Rust` with `Javascript`.

[![ppzFBHf.jpg](https://s1.ax1x.com/2023/04/14/ppzFBHf.jpg)](https://imgse.com/i/ppzFBHf)

- Checkpoint 1: CPU class structure finished, Render finished.
- Checkpoint 2: Main loop, Rom-loading class, Fonts-loading class, Random Number Generator, Fetch, finish all decode patterns.
- Final: All finished.
### Possible Challenges:
 working with a UI in Rust; decode every pattern correctly, tricky type casting among `usize`/`u8`/`u32`,time limit.
## References: 
_most useful:_ The book [An Introduction to Chip-8 Emulation using the Rust Programming Language](https://aquova.net/chip8/chip8.pdf)

[Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)

[Cowgod's Chip-8 Technical Reference v1.0](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#memmap)

[CHIP‐8 Technical Reference](https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference)
# Chip8Emu-2.0
