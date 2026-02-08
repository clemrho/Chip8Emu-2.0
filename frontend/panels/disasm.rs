use eframe::egui;

use crate::core::state::CpuState;

#[allow(dead_code)]
pub fn show_disasm(ui: &mut egui::Ui, state: &CpuState) {
    let pc = state.pc as usize;
    if pc + 1 < state.ram.len() {
        let op = ((state.ram[pc] as u16) << 8) | state.ram[pc + 1] as u16;
        ui.monospace(format!("PC: 0x{pc:04X}"));
        ui.monospace(format!("Next opcode: 0x{op:04X}"));
    } else {
        ui.monospace(format!("PC: 0x{pc:04X}"));
        ui.label("Next opcode: <out of bounds>");
    }
}
