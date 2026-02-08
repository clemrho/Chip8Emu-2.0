use eframe::egui;

use crate::core::state::CpuState;

#[allow(dead_code)]
pub fn show_memory(ui: &mut egui::Ui, state: &CpuState) {
    let pc = state.pc as usize;
    let start = pc.saturating_sub(32) & !0xF;
    let end = (start + 128).min(state.ram.len());

    ui.monospace("Addr  00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F");
    let mut addr = start;
    while addr < end {
        let mut line = format!("{addr:04X} ");
        for i in 0..16 {
            if addr + i < state.ram.len() {
                line.push_str(&format!("{:02X} ", state.ram[addr + i]));
            } else {
                line.push_str(".. ");
            }
        }
        ui.monospace(line);
        addr += 16;
    }
}
