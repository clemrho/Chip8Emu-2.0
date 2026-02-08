use eframe::egui;

// Placeholder for timeline UI
pub struct Timeline {}

impl Timeline {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, running: bool, cycles_executed: u64) {
        ui.horizontal(|ui| {
            ui.label("Timeline");
            ui.separator();
            let status = if running { "Running" } else { "Paused" };
            ui.small(format!("Status: {status}"));
            ui.separator();
            ui.small(format!("Cycles: {cycles_executed}"));
        });
    }
}
