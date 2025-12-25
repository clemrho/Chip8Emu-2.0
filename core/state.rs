use crate::core::utils::{Dstatus, Kstatus, FONTSET};

pub const RAM_SIZE: usize = 4096;
pub const V_REG_COUNT: usize = 16;
pub const STACK_SIZE: usize = 16;
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const KEY_COUNT: usize = 16;

pub struct CpuState {
    pub ram: [u8; RAM_SIZE],
    pub v_reg: [u8; V_REG_COUNT],
    pub stack: [u16; STACK_SIZE],
    pub sound_tmr: u8,
    pub delay_tmr: u8,
    pub i_reg: u16,
    pub sp: u16,
    pub pc: u16,
    pub screen: [Dstatus; SCREEN_WIDTH * SCREEN_HEIGHT],
    pub keys: [Kstatus; KEY_COUNT],
}

impl CpuState {
    pub fn new() -> Self {
        let mut state = Self {
            ram: [0; RAM_SIZE],
            v_reg: [0; V_REG_COUNT],
            stack: [0; STACK_SIZE],
            sound_tmr: 0,
            delay_tmr: 0,
            i_reg: 0,
            sp: 0,
            pc: 0x200,
            screen: [Dstatus::Off; SCREEN_WIDTH * SCREEN_HEIGHT],
            keys: [Kstatus::Default; KEY_COUNT],
        };
        // Load fontset
        state.ram[0..80].copy_from_slice(&FONTSET);
        state
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}
