use crate::core::state::CpuState;
use crate::core::utils::{Dstatus, Kstatus};
use rand::Rng;

pub struct Cpu {
    pub state: CpuState,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            state: CpuState::new(),
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        let start = 0x200;
        let end = start + data.len();
        if end <= self.state.ram.len() {
            self.state.ram[start..end].copy_from_slice(data);
        }
    }

    pub fn tick(&mut self) {
        let opcode = self.fetch();
        self.execute(opcode);
    }

    pub fn tick_timers(&mut self) {
        if self.state.delay_tmr > 0 {
            self.state.delay_tmr -= 1;
        }
        if self.state.sound_tmr > 0 {
            self.state.sound_tmr -= 1;
        }
    }

    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        if idx < 16 {
            self.state.keys[idx] = if pressed { Kstatus::Pressed } else { Kstatus::Default };
        }
    }

    fn fetch(&mut self) -> u16 {
        let pc = self.state.pc as usize;
        let b1 = self.state.ram[pc] as u16;
        let b2 = self.state.ram[pc + 1] as u16;
        self.state.pc += 2;
        (b1 << 8) | b2
    }

    fn execute(&mut self, op: u16) {
        let d1 = (op & 0xF000) >> 12;
        let d2 = (op & 0x0F00) >> 8;
        let d3 = (op & 0x00F0) >> 4;
        let d4 = op & 0x000F;
        
        let nnn = op & 0x0FFF;
        let nn = (op & 0x00FF) as u8;
        let x = d2 as usize;
        let y = d3 as usize;
        let n = d4 as usize;

        match (d1, d2, d3, d4) {
            (0, 0, 0, 0) => return,
            (0, 0, 0xE, 0) => self.state.screen = [Dstatus::Off; 64 * 32], // CLS
            (0, 0, 0xE, 0xE) => { // RET
                self.state.sp -= 1;
                self.state.pc = self.state.stack[self.state.sp as usize];
            },
            (1, _, _, _) => self.state.pc = nnn, // JP addr
            (2, _, _, _) => { // CALL addr
                self.state.stack[self.state.sp as usize] = self.state.pc;
                self.state.sp += 1;
                self.state.pc = nnn;
            },
            (3, _, _, _) => if self.state.v_reg[x] == nn { self.state.pc += 2; }, // SE Vx, byte
            (4, _, _, _) => if self.state.v_reg[x] != nn { self.state.pc += 2; }, // SNE Vx, byte
            (5, _, _, 0) => if self.state.v_reg[x] == self.state.v_reg[y] { self.state.pc += 2; }, // SE Vx, Vy
            (6, _, _, _) => self.state.v_reg[x] = nn, // LD Vx, byte
            (7, _, _, _) => self.state.v_reg[x] = self.state.v_reg[x].wrapping_add(nn), // ADD Vx, byte
            (8, _, _, _) => match d4 {
                0 => self.state.v_reg[x] = self.state.v_reg[y],
                1 => self.state.v_reg[x] |= self.state.v_reg[y],
                2 => self.state.v_reg[x] &= self.state.v_reg[y],
                3 => self.state.v_reg[x] ^= self.state.v_reg[y],
                4 => {
                    let (res, overflow) = self.state.v_reg[x].overflowing_add(self.state.v_reg[y]);
                    self.state.v_reg[x] = res;
                    self.state.v_reg[0xF] = if overflow { 1 } else { 0 };
                },
                5 => {
                    let (res, borrow) = self.state.v_reg[x].overflowing_sub(self.state.v_reg[y]);
                    self.state.v_reg[x] = res;
                    self.state.v_reg[0xF] = if borrow { 0 } else { 1 };
                },
                6 => {
                    self.state.v_reg[0xF] = self.state.v_reg[x] & 1;
                    self.state.v_reg[x] >>= 1;
                },
                7 => {
                    let (res, borrow) = self.state.v_reg[y].overflowing_sub(self.state.v_reg[x]);
                    self.state.v_reg[x] = res;
                    self.state.v_reg[0xF] = if borrow { 0 } else { 1 };
                },
                0xE => {
                    self.state.v_reg[0xF] = (self.state.v_reg[x] >> 7) & 1;
                    self.state.v_reg[x] <<= 1;
                },
                _ => {}
            },
            (9, _, _, 0) => if self.state.v_reg[x] != self.state.v_reg[y] { self.state.pc += 2; },
            (0xA, _, _, _) => self.state.i_reg = nnn,
            (0xB, _, _, _) => self.state.pc = nnn + self.state.v_reg[0] as u16,
            (0xC, _, _, _) => self.state.v_reg[x] = rand::random::<u8>() & nn,
            (0xD, _, _, _) => self.draw_sprite(x, y, n),
            (0xE, _, 9, 0xE) => if self.state.keys[self.state.v_reg[x] as usize] == Kstatus::Pressed { self.state.pc += 2; },
            (0xE, _, 0xA, 1) => if self.state.keys[self.state.v_reg[x] as usize] != Kstatus::Pressed { self.state.pc += 2; },
            (0xF, _, 0, 7) => self.state.v_reg[x] = self.state.delay_tmr,
            (0xF, _, 0, 0xA) => {
                // Wait for key press (blocking-ish, but we just decrement PC to retry)
                let mut pressed = false;
                for (i, k) in self.state.keys.iter().enumerate() {
                    if *k == Kstatus::Pressed {
                        self.state.v_reg[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    self.state.pc -= 2;
                }
            },
            (0xF, _, 1, 5) => self.state.delay_tmr = self.state.v_reg[x],
            (0xF, _, 1, 8) => self.state.sound_tmr = self.state.v_reg[x],
            (0xF, _, 1, 0xE) => self.state.i_reg = self.state.i_reg.wrapping_add(self.state.v_reg[x] as u16),
            (0xF, _, 2, 9) => self.state.i_reg = self.state.v_reg[x] as u16 * 5,
            (0xF, _, 3, 3) => {
                let val = self.state.v_reg[x];
                self.state.ram[self.state.i_reg as usize] = val / 100;
                self.state.ram[self.state.i_reg as usize + 1] = (val % 100) / 10;
                self.state.ram[self.state.i_reg as usize + 2] = val % 10;
            },
            (0xF, _, 5, 5) => {
                for i in 0..=x {
                    self.state.ram[self.state.i_reg as usize + i] = self.state.v_reg[i];
                }
            },
            (0xF, _, 6, 5) => {
                for i in 0..=x {
                    self.state.v_reg[i] = self.state.ram[self.state.i_reg as usize + i];
                }
            },
            _ => {}
        }
    }

    fn draw_sprite(&mut self, x_idx: usize, y_idx: usize, height: usize) {
        let x_coord = self.state.v_reg[x_idx] as u16;
        let y_coord = self.state.v_reg[y_idx] as u16;
        self.state.v_reg[0xF] = 0;

        for y_line in 0..height {
            let addr = self.state.i_reg + y_line as u16;
            if addr as usize >= self.state.ram.len() { continue; }
            let pixels = self.state.ram[addr as usize];
            
            for x_line in 0..8 {
                if (pixels & (0x80 >> x_line)) != 0 {
                    let x = (x_coord + x_line) as usize % 64;
                    let y = (y_coord + y_line as u16) as usize % 32;
                    let idx = x + 64 * y;
                    
                    if self.state.screen[idx] == Dstatus::On {
                        self.state.v_reg[0xF] = 1;
                        self.state.screen[idx] = Dstatus::Off;
                    } else {
                        self.state.screen[idx] = Dstatus::On;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let cpu = Cpu::new();
        assert_eq!(cpu.state.pc, 0x200);
        assert_eq!(cpu.state.sp, 0);
    }

    #[test]
    fn test_opcode_6xnn_set_vx() {
        let mut cpu = Cpu::new();
        // 61AA -> V1 = 0xAA
        cpu.execute(0x61AA);
        assert_eq!(cpu.state.v_reg[1], 0xAA);
    }

    #[test]
    fn test_opcode_7xnn_add_vx() {
        let mut cpu = Cpu::new();
        // 6110 -> V1 = 0x10
        cpu.execute(0x6110);
        // 7110 -> V1 += 0x10
        cpu.execute(0x7110);
        assert_eq!(cpu.state.v_reg[1], 0x20);
    }
}
