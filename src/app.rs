use crate::gameboy::Gameboy;
use anyhow::Result;
use egui::{Pos2, Ui, Vec2, vec2};
use crate::error;
use std::env;

const SOURCE_CODE_LINK: &str = "https://github.com/hferraud/gbmu/";

const GAMEBOY_SCREEN_WIDTH: u32 = 160;
const GAMEBOY_SCREEN_HEIGHT: u32 = 144;

const DEFAULT_GAME_PANEL_WIDTH_RATIO: f32 = 0.55;
const DEFAULT_INSTRUCTION_PANEL_WIDTH_RATIO: f32 = 0.55;
const DEFAULT_REGISTERS_PANEL_HEIGHT_RATIO: f32 = 0.3;

pub struct App {
    gameboy: Option<Gameboy>,
}

impl App {
    pub fn new(
        _cc: &eframe::CreationContext<'_>,
    ) -> Self {

        // TODO remove this
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            println!("Usage: ./{} <ROM>", args[0]);
            panic!("{}", error::invalid_argument());
        }
        let rom_path = &args[1];
        let gameboy = Gameboy::new(rom_path)
            .expect("Invalid ROM file");
        // !

        Self {
            // TODO set gameboy to None at creation
            gameboy: Some(gameboy),
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
                #[cfg(not(target_arch = "wasm32"))] {
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
                ui.label("Registers");

                ui.allocate_space(ui.available_size());
            });
    }

    fn render_ram_panel(&mut self, memory_panel: &mut Ui) {
        let available_height = memory_panel.available_height();
        egui::TopBottomPanel::bottom("RAM panel")
            .resizable(false)
            .default_height(available_height)
            .show_inside(memory_panel, |ui| {
                ui.set_height(available_height);

                ui.label("RAM");

                ui.allocate_space(ui.available_size());
            });
    }
}
