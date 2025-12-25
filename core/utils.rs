// some utilities written here
pub const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

#[derive(Copy,Clone,PartialEq)]
pub enum Dstatus{
    On,
    Off,
}

#[derive(Copy,Clone,PartialEq)]
pub enum Kstatus{
    Default,
    Pressed,
}

// pub struct Keyboard{
//     input_char : char,
// }

// impl Keyboard {
//     /*  transfer modern keyboard map below:
//             1   2   3   4
//             Q   W   E   R
//             A   S   D   F
//             Z   X   C   V
//      to the chip-8 keyboard map:
//             1   2   3   C
//             4   5   6   D
//             7   8   9   E
//             A   0   B   F
//     clicking all other keys will cause it to enter '0'.
//      */
//     pub fn char_injection(self) -> char {
//         let output: char = match self.input_char {
//             '1' => '1',
//             '2' => '2',
//             '3' => '3',
//             '4' => 'C',
//             'Q' => '4',
//             'W' => '5',
//             'E' => '6',
//             'R' => 'D',
//             'A' => '7',
//             'S' => '8',
//             'D' => '9',
//             'F' => 'E',
//             'Z' => 'A',
//             'X' => '0',
//             'C' => 'B',
//             'V' => 'F',
//             _ => '0',
//         };
//     output
//     }
// }