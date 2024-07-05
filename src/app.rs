use crate::gameboy::Gameboy;
use anyhow::Result;
use egui::{Pos2, Ui, Vec2, vec2};
use crate::error;
use std::env;

const GAMEBOY_SCREEN_WIDTH: u32 = 160;
const GAMEBOY_SCREEN_HEIGHT: u32 = 144;

const DEFAULT_GAME_PANEL_WIDTH_RATIO: f32 = 0.7;

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
        self.render_bottom_panel(ctx);
        self.render_central_panel(ctx);
    }
}

impl App {
    fn render_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
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
            ui.add(egui::github_link_file!(
                "https://github.com/hferraud/gbmu/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });
    }

    fn render_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_game_panel(ctx, ui);
            self.render_debugger_panel(ctx, ui);
        });
    }

    fn render_game_panel(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        egui::SidePanel::left("Game panel")
            .resizable(true)
            .default_width(ui.available_width() * DEFAULT_GAME_PANEL_WIDTH_RATIO)
            .show(ctx, |ui| {
                ui.label("Game");
                ui.allocate_space(ui.available_size());
            });
    }

    fn render_debugger_panel(&mut self, ctx: &egui::Context, _ui: &mut Ui) {
        egui::SidePanel::right("Debugger panel")
            .resizable(false)
            .default_width(f32::MAX)
            .show(ctx, |ui| {
                ui.label("Debugger");
                ui.set_width(f32::MAX);
                ui.allocate_space(ui.available_size());
            });
    }
}
