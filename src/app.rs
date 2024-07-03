
use crate::gameboy::Gameboy;
use anyhow::Result;
use egui::{Pos2, Ui, Vec2};
use crate::error;
use std::env;

const GAMEBOY_SCREEN_WIDTH: u32 = 160;
const GAMEBOY_SCREEN_HEIGHT: u32 = 144;



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
        self.render_central_panet(ctx);
        self.render_bottom_panel(ctx);
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

    fn render_central_panet(&mut self, ctx: &egui::Context) {
        let Some(gameboy) = &self.gameboy else {
            // TODO handle gameboy is not loaded
            return;
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Debugger");
            self.render_game_window(ctx, ui);
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

    fn render_game_window(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        let available_size = ui.available_size();

        
        egui::Window::new("Game container")
            .movable(false)
            .collapsible(false)
            .title_bar(false)
            .resizable([true, false])
            .current_pos([0., 0.])
            // TODO do not hard code 0.7 it will depend on debugger size
            .default_size([available_size.x * 0.7, available_size.y])
            .show(ctx, |ui| {
                ui.label("Game");
                ui.allocate_space(ui.available_size());
            });
    }
}
