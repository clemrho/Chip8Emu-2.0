mod core;
mod frontend;

use frontend::app::Chip8App;

fn main() -> eframe::Result<()> {
    env_logger::init();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Chip8 Sandbox",
        native_options,
        Box::new(|cc| Ok(Box::new(Chip8App::new(cc)))),
    )
}
