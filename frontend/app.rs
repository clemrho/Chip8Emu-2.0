use eframe::egui;

pub struct Chip8App {
    // Placeholder for app state
}

impl Chip8App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals
        Self {
            
        }
    }
}

impl eframe::App for Chip8App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Chip8 Sandbox");
            ui.label("Welcome to the native macOS Chip-8 Emulator!");
        });
    }
}
