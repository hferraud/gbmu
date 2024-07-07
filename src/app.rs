use crate::error;
use crate::gameboy::Gameboy;
use anyhow::Result;
use egui::{vec2, Pos2, ScrollArea, Ui, Vec2};
use std::env;

const SOURCE_CODE_LINK: &str = "https://github.com/hferraud/gbmu/";

const ROM_IS_NOT_INSERTED: &str = "ROM is not inserted";

const GAMEBOY_SCREEN_WIDTH: u32 = 160;
const GAMEBOY_SCREEN_HEIGHT: u32 = 144;

const DEFAULT_GAME_PANEL_WIDTH_RATIO: f32 = 0.55;
const DEFAULT_INSTRUCTION_PANEL_WIDTH_RATIO: f32 = 0.55;
const DEFAULT_REGISTERS_PANEL_HEIGHT_RATIO: f32 = 0.3;

#[derive(Default)]
pub struct App {
    gameboy: Option<Gameboy>,

    selected_ram: RamTypes,
}

enum RamTypes {
    MBC0,
    WRAM,
    HRAM,
}

impl Default for RamTypes {
    fn default() -> Self {
        Self::MBC0
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // TODO remove this
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            println!("Usage: ./{} <ROM>", args[0]);
            panic!("{}", error::invalid_argument());
        }
        let rom_path = &args[1];
        let gameboy = Gameboy::new(rom_path).expect("Invalid ROM file");
        // !

        Self {
            // TODO set gameboy to None at creation
            gameboy: Some(gameboy),
            ..Default::default()
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.render_top_panel(ctx);
        self.render_central_panel(ctx);
        self.render_bottom_panel(ctx);
    }
}

impl App {
    fn render_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
    }

    fn render_bottom_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.hyperlink_to("Source code", SOURCE_CODE_LINK);
                egui::warn_if_debug_build(ui);
            });
        });
    }

    fn render_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_game_panel(ui);
            self.render_debugger_panel(ui);
        });
    }

    fn render_game_panel(&mut self, central_panel: &mut Ui) {
        egui::SidePanel::left("Game panel")
            .resizable(true)
            .default_width(central_panel.available_width() * DEFAULT_GAME_PANEL_WIDTH_RATIO)
            .show_inside(central_panel, |ui| {
                ui.label("Game");

                ui.allocate_space(ui.available_size());
            });
    }

    fn render_debugger_panel(&mut self, central_panel: &mut Ui) {
        let available_width = central_panel.available_width();
        egui::SidePanel::right("Debugger panel")
            .resizable(false)
            .default_width(available_width)
            .show_inside(central_panel, |ui| {
                ui.set_width(available_width);

                self.render_instructions_panel(ui);
                self.render_memory_panel(ui);

                ui.allocate_space(ui.available_size());
            });
    }

    fn render_instructions_panel(&mut self, debugger_panel: &mut Ui) {
        egui::SidePanel::left("Instructions panel")
            .resizable(true)
            .default_width(debugger_panel.available_width() * DEFAULT_INSTRUCTION_PANEL_WIDTH_RATIO)
            .show_inside(debugger_panel, |ui| {
                ui.label("Instructions");

                ui.allocate_space(ui.available_size());
            });
    }

    fn render_memory_panel(&mut self, debugger_panel: &mut Ui) {
        let available_width = debugger_panel.available_width();
        egui::SidePanel::right("Memory panel")
            .resizable(false)
            .default_width(available_width)
            .show_inside(debugger_panel, |ui| {
                ui.set_width(available_width);

                self.render_registers_panel(ui);
                self.render_ram_panel(ui);

                ui.allocate_space(ui.available_size());
            });
    }

    fn render_registers_panel(&mut self, memory_panel: &mut Ui) {
        egui::TopBottomPanel::top("Registers panel")
            .resizable(true)
            .default_height(memory_panel.available_height() * DEFAULT_REGISTERS_PANEL_HEIGHT_RATIO)
            .show_inside(memory_panel, |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);

                ui.label("Registers");
                ui.add(egui::Separator::default().horizontal());

                egui::ScrollArea::both().show(ui, |ui| {
                    if let Some(gameboy) = &self.gameboy {
                        self.render_cpu_registers(ui, gameboy);
                    } else {
                        ui.label(ROM_IS_NOT_INSERTED);
                    }
                });
            });
    }

    fn render_cpu_registers(&self, registers_panel: &mut Ui, gameboy: &Gameboy) {
        registers_panel.label(format!(
            "a: {:02X}\t\tb: {:02X}",
            gameboy.cpu.registers.a, gameboy.cpu.registers.b
        ));
        registers_panel.label(format!(
            "c: {:02X}\t\td: {:02X}",
            gameboy.cpu.registers.c, gameboy.cpu.registers.d
        ));
        registers_panel.label(format!(
            "e: {:02X}\t\tf: {:02X}",
            gameboy.cpu.registers.e, gameboy.cpu.registers.f
        ));
        registers_panel.label(format!(
            "h: {:02X}\t\tl: {:02X}",
            gameboy.cpu.registers.h, gameboy.cpu.registers.l
        ));

        registers_panel.label(format!(
            "pc: {:04X}\tsp: {:04X}",
            gameboy.cpu.registers.pc, gameboy.cpu.registers.sp
        ));
        registers_panel.label(format!("ime: {}", gameboy.cpu.ime));
    }

    fn render_ram_panel(&mut self, memory_panel: &mut Ui) {
        let available_height = memory_panel.available_height();
        egui::TopBottomPanel::bottom("RAM panel")
            .resizable(false)
            .default_height(available_height)
            .show_inside(memory_panel, |ui| {
                ui.set_height(available_height);

                ui.label("RAM: {}");
                ui.add(egui::Separator::default().horizontal());

                egui::ScrollArea::both().show(ui, |ui| {
                    self.render_ram(ui);
                });

                ui.allocate_space(ui.available_size());
            });
    }

    fn render_ram(&mut self, ram_panel: &mut Ui) {
        let Some(gameboy) = &mut self.gameboy else {
            ram_panel.label(ROM_IS_NOT_INSERTED);
            return;
        };

        for address in 0..0xFFFF {

            let Ok(word) = gameboy.mmu.get_word(address) else {
                ram_panel.label(format!("{:04X}: None", address));
                continue;
            };
            ram_panel.label(format!("{:04X}: {:04X}", address, word));
        }
    }
}
