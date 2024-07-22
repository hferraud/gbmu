mod game_data;
mod instruction_map;

use crate::app::instruction_map::Instruction;
use crate::error;
use crate::gameboy::Gameboy;
use anyhow::Result;
use egui::{pos2, vec2, Align2, FontDefinitions, Pos2, Rect, ScrollArea, Sense, TextStyle, Ui, Vec2, Stroke, Color32, Rounding};
use game_data::{GameData, RunStatus};
use instruction_map::InstructionMap;
use std::env;
use crate::lcd::{GAMEBOY_SCREEN_WIDTH, GAMEBOY_SCREEN_HEIGHT};

const SOURCE_CODE_LINK: &str = "https://github.com/hferraud/gbmu/";

const ROM_IS_NOT_INSERTED: &str = "ROM is not inserted";

const DEFAULT_GAME_PANEL_WIDTH_RATIO: f32 = 0.55;
const DEFAULT_INSTRUCTION_PANEL_WIDTH_RATIO: f32 = 0.55;
const DEFAULT_REGISTERS_PANEL_HEIGHT_RATIO: f32 = 0.3;

const RAM_DUMP_STEP: usize = 16;

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
        let mut fonts = FontDefinitions::default();
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .clear();
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .push("Hack".to_string());
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .push("emoji-icon-font".to_string());
        ctx.set_fonts(fonts);
        self.render_top_panel(ctx);
        self.render_bottom_panel(ctx);
        self.render_central_panel(ctx);
        ctx.request_repaint();
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
                // game_data.gameboy.lcd.display(ui);
                let available_size = ui.available_size();
                let ratio_x = (available_size.x / GAMEBOY_SCREEN_WIDTH as f32).floor();
                let ratio_y = (available_size.y / GAMEBOY_SCREEN_HEIGHT as f32).floor();
                let ratio = ratio_x.min(ratio_y);
                let painter_size = Vec2::new(GAMEBOY_SCREEN_WIDTH as f32 * ratio, GAMEBOY_SCREEN_HEIGHT as f32 * ratio);
                let pixel_size = Vec2::splat(ratio);
                let (response, painter) = ui.allocate_painter(
                    painter_size, Sense::focusable_noninteractive()
                );

                let viewport = response.rect;
                for (x, line) in game_data.gameboy.lcd.iter().enumerate() {
                    for (y, color) in line.iter().enumerate() {
                        let position = Pos2 {
                            x: viewport.min.x + pixel_size.x * x as f32,
                            y: viewport.min.y + pixel_size.y * y as f32,
                        };
                        let pixel = Rect::from_min_size(position, pixel_size);
                        let (r, g, b) = color;
                        let fill_color = Color32::from_rgb(*r, *g, *b);
                        painter.rect_filled(pixel, Rounding::ZERO, fill_color);
                    }
                }

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

    fn render_instructions_panel(debugger_panel: &mut Ui, game_data: &mut GameData) {
        egui::SidePanel::left("Instructions panel")
            .resizable(true)
            .default_width(debugger_panel.available_width() * DEFAULT_INSTRUCTION_PANEL_WIDTH_RATIO)
            .show_inside(debugger_panel, |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);

                ui.horizontal(|ui| {
                    ui.label("Instructions:");
                    if game_data.run_status == RunStatus::Running {
                        // TODO handle error
                        let _ = game_data.gameboy.cpu.run(&mut game_data.gameboy.mmu);
                        if game_data
                            .breakpoints
                            .contains(&game_data.gameboy.cpu.registers.pc)
                        {
                            game_data.run_status = RunStatus::Waiting;
                        }
                    } else if ui.button("🔁").clicked() {
                        // TODO handle error
                        let _ = game_data.gameboy.cpu.run(&mut game_data.gameboy.mmu);
                    } else if ui.button("▶️").clicked() {
                        game_data.run_status = RunStatus::Running;
                    }
                });
                ui.add(egui::Separator::default().horizontal());

                Self::render_instructions(ui, game_data);

                ui.allocate_space(ui.available_size());
            });
    }

    fn render_instructions(instructions_panel: &mut Ui, game_data: &mut GameData) {
        let font_id = TextStyle::Body.resolve(instructions_panel.style());
        let row_height = instructions_panel.fonts(|f| f.row_height(&font_id));
        egui::ScrollArea::both().auto_shrink([false; 2]).show_rows(
            instructions_panel,
            row_height,
            game_data.instructions.len(),
            |ui, row_range| {
                for row in row_range {
                    let (pc, instruction) = &game_data.instructions[row];
                    let breakpoint_selected = game_data.breakpoints.contains(pc);
                    ui.horizontal(|ui| {
                        let text = format!("{:04X}\t{}", pc, instruction);

                        if ui.radio(breakpoint_selected, "").clicked() {
                            if breakpoint_selected {
                                game_data.breakpoints.remove(pc);
                            } else {
                                game_data.breakpoints.insert(*pc);
                            }
                        }
                        let color = if game_data.breakpoints.contains(pc) {
                            egui::Color32::RED
                        } else if game_data.gameboy.cpu.registers.pc == *pc {
                            if ui.style().visuals.dark_mode {
                                egui::Color32::DARK_GRAY
                            } else {
                                egui::Color32::GRAY
                            }
                        } else {
                            egui::Color32::TRANSPARENT
                        };
                        ui.label(egui::RichText::new(text).background_color(color));
                    });
                }
            },
        );
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

                egui::ScrollArea::both()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        Self::render_ram(ui, &mut game_data.gameboy);
                    });

                ui.allocate_space(ui.available_size());
            });
    }

    fn render_ram(ram_panel: &mut Ui, gameboy: &mut Gameboy) {
        let font_id = TextStyle::Body.resolve(ram_panel.style());
        let row_height = ram_panel.fonts(|f| f.row_height(&font_id));
        const NUM_ROWS: usize = 0xFFFF / RAM_DUMP_STEP;
        egui::ScrollArea::both().auto_shrink([false; 2]).show_rows(
            ram_panel,
            row_height,
            NUM_ROWS,
            |ui, row_range| {
                for row in row_range {
                    ui.horizontal(|ui| {
                        ui.label(Self::create_ram_display_line(gameboy, row * RAM_DUMP_STEP));
                    });
                }
            },
        );
    }

    fn create_ram_display_line(gameboy: &mut Gameboy, address: usize) -> String {
        let memory = (address..(address + RAM_DUMP_STEP))
            .into_iter()
            .map(|address| Ok(gameboy.mmu.get_word(address)?))
            .collect::<Result<Vec<u8>>>();

        let Ok(memory) = memory else {
            return format!("{address:04X}: Unavailable");
        };

        let fold_fn = |acc, elem| acc + format!(" {elem:02X}").as_str();
        let data = memory
            .iter()
            .take(RAM_DUMP_STEP / 2)
            .fold(String::new(), fold_fn)
            + " "
            + &memory
                .iter()
                .skip(RAM_DUMP_STEP / 2)
                .fold(String::new(), fold_fn);

        let ascii = memory.iter().fold(String::new(), |mut acc, elem| {
            let c = *elem as char;
            let c = if c.is_ascii_graphic() { c } else { '.' };
            acc.push(c);
            acc
        });
        format!("{address:04X} {data}  |{ascii}|")
    }
}
