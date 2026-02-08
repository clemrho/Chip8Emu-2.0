use eframe::egui;

use crate::core::state::CpuState;

#[allow(dead_code)]
pub fn show_registers(ui: &mut egui::Ui, state: &CpuState) {
    egui::Grid::new("registers_grid")
        .striped(true)
        .num_columns(2)
        .show(ui, |ui| {
            for i in 0..state.v_reg.len() {
                ui.label(format!("V{:X}", i));
                ui.monospace(format!("0x{:02X}", state.v_reg[i]));
                ui.end_row();
            }
            ui.label("I");
            ui.monospace(format!("0x{:04X}", state.i_reg));
            ui.end_row();
            ui.label("PC");
            ui.monospace(format!("0x{:04X}", state.pc));
            ui.end_row();
            ui.label("SP");
            ui.monospace(format!("0x{:02X}", state.sp));
            ui.end_row();
            ui.label("DT");
            ui.monospace(format!("0x{:02X}", state.delay_tmr));
            ui.end_row();
            ui.label("ST");
            ui.monospace(format!("0x{:02X}", state.sound_tmr));
            ui.end_row();
        });
}
