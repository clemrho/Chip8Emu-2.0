use std::fs;
use std::path::Path;

use eframe::egui;

use super::panels::{disasm, memory, registers};
use super::timeline::Timeline;
use crate::core::state::{CpuState, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::core::step::Cpu;

pub struct Chip8App {
    timeline: Timeline,
    cpu: Cpu,
    running: bool,
    rom_path: String,
    last_error: Option<String>,
    cycles_per_frame: u32,
    cycles_executed: u64,
    screen_scale: f32,
}

impl Chip8App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals
        Self {
            timeline: Timeline::new(),
            cpu: Cpu::new(),
            running: false,
            rom_path: String::new(),
            last_error: None,
            cycles_per_frame: 10,
            cycles_executed: 0,
            screen_scale: 10.0,
        }
    }

    fn can_fetch(&self) -> bool {
        let pc = self.cpu.state.pc as usize;
        pc + 1 < self.cpu.state.ram.len()
    }

    fn load_rom_from_path(&mut self, path: &str) {
        match fs::read(path) {
            Ok(data) => {
                self.cpu.state.reset();
                self.cpu.load_rom(&data);
                self.running = false;
                self.cycles_executed = 0;
                self.last_error = None;
                self.rom_path = path.to_string();
            }
            Err(err) => {
                self.last_error = Some(err.to_string());
            }
        }
    }

    fn draw_screen(ui: &mut egui::Ui, state: &CpuState, scale: f32) {
        let width = SCREEN_WIDTH as f32 * scale;
        let height = SCREEN_HEIGHT as f32 * scale;
        let (rect, _) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
        let painter = ui.painter_at(rect);

        painter.rect_filled(rect, 0.0, egui::Color32::BLACK);
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = x + y * SCREEN_WIDTH;
                if state.screen[idx] == crate::core::utils::Dstatus::On {
                    let px = rect.left() + x as f32 * scale;
                    let py = rect.top() + y as f32 * scale;
                    let pixel_rect =
                        egui::Rect::from_min_size(egui::pos2(px, py), egui::vec2(scale, scale));
                    painter.rect_filled(pixel_rect, 0.0, egui::Color32::WHITE);
                }
            }
        }
    }
}

impl eframe::App for Chip8App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.running {
            for _ in 0..self.cycles_per_frame {
                if !self.can_fetch() {
                    self.running = false;
                    self.last_error = Some(
                        "PC out of bounds. The program counter reached the end of RAM."
                            .to_string(),
                    );
                    break;
                }
                self.cpu.tick();
                self.cycles_executed += 1;
            }
            if self.running {
                self.cpu.tick_timers();
                ctx.request_repaint();
            }
        }

        egui::TopBottomPanel::top("top_bar")
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Chip8 Sandbox")
                            .size(28.0)
                            .color(egui::Color32::from_rgb(0, 122, 255)),
                    );
                    ui.separator();
                    ui.label(
                        egui::RichText::new("Native macOS Chip-8 Emulator")
                            .size(20.0)
                            .color(egui::Color32::from_rgb(0, 102, 204)),
                    );
                });
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    let run_label = if self.running { "Pause" } else { "Run" };
                    if ui.button(run_label).clicked() {
                        self.running = !self.running;
                    }
                    if ui.button("Step").clicked() {
                        if self.can_fetch() {
                            self.cpu.tick();
                            self.cpu.tick_timers();
                            self.cycles_executed += 1;
                        } else {
                            self.running = false;
                            self.last_error = Some(
                                "PC out of bounds. The program counter reached the end of RAM."
                                    .to_string(),
                            );
                        }
                    }
                    if ui.button("Reset").clicked() {
                        self.cpu.state.reset();
                        self.running = false;
                        self.cycles_executed = 0;
                    }
                    ui.separator();
                    ui.label("ROM Path:");
                    ui.text_edit_singleline(&mut self.rom_path);
                    if ui.button("Load ROM").clicked() {
                        let path = self.rom_path.trim().to_string();
                        self.load_rom_from_path(&path);
                    }
                    ui.separator();
                    ui.label("Quick tests:");
                    let suite_dir = Path::new("chip8-test-suite/bin");
                    let ibm = suite_dir.join("2-ibm-logo.ch8");
                    let corax = suite_dir.join("3-corax+.ch8");
                    let flags = suite_dir.join("4-flags.ch8");
                    if ui.button("IBM Logo").clicked() {
                        let path = ibm.to_string_lossy().into_owned();
                        self.load_rom_from_path(&path);
                    }
                    if ui.button("Corax+").clicked() {
                        let path = corax.to_string_lossy().into_owned();
                        self.load_rom_from_path(&path);
                    }
                    if ui.button("Flags").clicked() {
                        let path = flags.to_string_lossy().into_owned();
                        self.load_rom_from_path(&path);
                    }
                });
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label("Cycles/frame:");
                    ui.add(egui::Slider::new(&mut self.cycles_per_frame, 1..=100));
                    ui.separator();
                    ui.label("Screen scale:");
                    ui.add(egui::Slider::new(&mut self.screen_scale, 4.0..=20.0));
                    if let Some(err) = &self.last_error {
                        ui.colored_label(egui::Color32::RED, format!("Error: {err}"));
                    }
                });
            });

        egui::TopBottomPanel::bottom("timeline_bar")
            .resizable(false)
            .show(ctx, |ui| {
                self.timeline
                    .show(ui, self.running, self.cycles_executed);
            });

        egui::SidePanel::left("registers_panel")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Registers");
                ui.separator();
                registers::show_registers(ui, &self.cpu.state);
            });

        egui::SidePanel::right("memory_panel")
            .resizable(true)
            .default_width(240.0)
            .show(ctx, |ui| {
                ui.heading("Memory");
                ui.separator();
                memory::show_memory(ui, &self.cpu.state);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Screen");
            ui.separator();
            Self::draw_screen(ui, &self.cpu.state, self.screen_scale);
            ui.add_space(12.0);
            ui.heading("Disassembly");
            ui.separator();
            disasm::show_disasm(ui, &self.cpu.state);
        });
    }
}
