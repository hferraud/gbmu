mod game_data;
mod instruction_map;

use crate::app::instruction_map::Instruction;
use crate::error;
use crate::gameboy::Gameboy;
use anyhow::Result;
use egui::{vec2, Pos2, ScrollArea, Ui, Vec2};
use game_data::GameData;
use instruction_map::InstructionMap;
use std::env;

const SOURCE_CODE_LINK: &str = "https://github.com/hferraud/gbmu/";

const ROM_IS_NOT_INSERTED: &str = "ROM is not inserted";

const GAMEBOY_SCREEN_WIDTH: u32 = 160;
const GAMEBOY_SCREEN_HEIGHT: u32 = 144;

const DEFAULT_GAME_PANEL_WIDTH_RATIO: f32 = 0.55;
const DEFAULT_INSTRUCTION_PANEL_WIDTH_RATIO: f32 = 0.55;
const DEFAULT_REGISTERS_PANEL_HEIGHT_RATIO: f32 = 0.3;

pub struct App {
    game_data: Option<GameData>,

    instruction_map: InstructionMap,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Result<Self> {
        let instruction_map = InstructionMap::new()?;
        Ok(Self {
            // TODO set game_data to None at creation
            game_data: Some(GameData::new("", &instruction_map)?),
            instruction_map,
        })
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
        let Some(game_data) = &mut self.game_data else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(ROM_IS_NOT_INSERTED);
                });
            });
            return;
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            Self::render_game_panel(ui, game_data);
            Self::render_debugger_panel(ui, game_data);
        });
    }

    fn render_game_panel(central_panel: &mut Ui, game_data: &GameData) {
        egui::SidePanel::left("Game panel")
            .resizable(true)
            .default_width(central_panel.available_width() * DEFAULT_GAME_PANEL_WIDTH_RATIO)
            .show_inside(central_panel, |ui| {
                ui.label("Game");

                ui.allocate_space(ui.available_size());
            });
    }

    fn render_debugger_panel(central_panel: &mut Ui, game_data: &mut GameData) {
        let available_width = central_panel.available_width();
        egui::SidePanel::right("Debugger panel")
            .resizable(false)
            .default_width(available_width)
            .show_inside(central_panel, |ui| {
                ui.set_width(available_width);

                Self::render_instructions_panel(ui, game_data);
                Self::render_memory_panel(ui, game_data);

                ui.allocate_space(ui.available_size());
            });
    }

    fn render_instructions_panel(debugger_panel: &mut Ui, game_data: &GameData) {
        egui::SidePanel::left("Instructions panel")
            .resizable(true)
            .default_width(debugger_panel.available_width() * DEFAULT_INSTRUCTION_PANEL_WIDTH_RATIO)
            .show_inside(debugger_panel, |ui| {
                ui.label("Instructions:");
                ui.add(egui::Separator::default().horizontal());

                egui::ScrollArea::both().show(ui, |ui| {
                    Self::render_instructions(ui, game_data);
                });

                ui.allocate_space(ui.available_size());
            });
    }

    fn render_instructions(instructions_panel: &mut Ui, game_data: &GameData) {
        let mut instructions = String::new();

        for (pc, instruction) in game_data.instructions.iter() {
            instructions += &format!("{:04x}/t{}\n", pc, instruction);
        }
        instructions_panel.label(instructions);
    }

    fn render_memory_panel(debugger_panel: &mut Ui, game_data: &mut GameData) {
        let available_width = debugger_panel.available_width();
        egui::SidePanel::right("Memory panel")
            .resizable(false)
            .default_width(available_width)
            .show_inside(debugger_panel, |ui| {
                ui.set_width(available_width);

                Self::render_registers_panel(ui, game_data);
                Self::render_ram_panel(ui, game_data);

                ui.allocate_space(ui.available_size());
            });
    }

    fn render_registers_panel(memory_panel: &mut Ui, game_data: &mut GameData) {
        egui::TopBottomPanel::top("Registers panel")
            .resizable(true)
            .default_height(memory_panel.available_height() * DEFAULT_REGISTERS_PANEL_HEIGHT_RATIO)
            .show_inside(memory_panel, |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);

                ui.label("Registers");
                ui.add(egui::Separator::default().horizontal());

                egui::ScrollArea::both().show(ui, |ui| {
                    Self::render_cpu_registers(ui, &mut game_data.gameboy);
                });
            });
    }

    fn render_cpu_registers(registers_panel: &mut Ui, gameboy: &mut Gameboy) {
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

    fn render_ram_panel(memory_panel: &mut Ui, game_data: &mut GameData) {
        let available_height = memory_panel.available_height();
        egui::TopBottomPanel::bottom("RAM panel")
            .resizable(false)
            .default_height(available_height)
            .show_inside(memory_panel, |ui| {
                ui.set_height(available_height);

                ui.label("RAM:");
                ui.add(egui::Separator::default().horizontal());

                egui::ScrollArea::both().show(ui, |ui| {
                    Self::render_ram(ui, &mut game_data.gameboy);
                });

                ui.allocate_space(ui.available_size());
            });
    }

    fn render_ram(ram_panel: &mut Ui, gameboy: &mut Gameboy) {
        let mut output = "".to_string();

        for address in (0..0xFFFF).step_by(2) {
            let Ok(word) = gameboy.mmu.get_dword(address) else {
                continue;
            };
            output += &format!("{:04X}: {:04X}\n", address, word);
        }

        ram_panel.label(output);
    }
}
