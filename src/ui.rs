use eframe::egui::{self, Color32, FontData, FontDefinitions, FontFamily, TextEdit, TextStyle};
use std::collections::VecDeque;
use std::process::Command;
use std::sync::Arc;

pub struct KaliTerminalApp {
    input: String,
    output: VecDeque<String>,
}

impl Default for KaliTerminalApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: VecDeque::from(vec![
                // No banner in output buffer
            ]),
        }
    }
}

impl eframe::App for KaliTerminalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "JetBrainsMono".to_owned(),
            Arc::new(FontData::from_static(include_bytes!("../assets/JetBrainsMono-Regular.ttf"))),
        );
        fonts
            .families
            .entry(FontFamily::Monospace)
            .or_default()
            .insert(0, "JetBrainsMono".to_owned());
        ctx.set_fonts(fonts);

        egui::TopBottomPanel::top("custom_top_bar").show(ctx, |ui| {
            // Custom draggable top bar
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                let circle_radius = 10.0;
                let spacing = 10.0;
                let y_center = ui.min_rect().center().y;
                let x_start = ui.min_rect().left() + 20.0;
                let colors = [Color32::RED, Color32::YELLOW, Color32::GREEN];
                let icons = ["×", "–", "□"];
                let mut clicked = [false; 3];
                let mut hovered = false;
                for (i, &color) in colors.iter().enumerate() {
                    let center = egui::pos2(x_start + i as f32 * (circle_radius * 2.0 + spacing), y_center);
                    let rect = egui::Rect::from_center_size(center, egui::vec2(circle_radius * 2.0, circle_radius * 2.0));
                    let response = ui.allocate_rect(rect, egui::Sense::click());
                    ui.painter().circle_filled(center, circle_radius, color);
                    // Draw bold, large, black icon, perfectly centered
                    ui.painter().text(center, egui::Align2::CENTER_CENTER, icons[i], egui::FontId::monospace(18.0), Color32::BLACK);
                    if response.hovered() {
                        ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                        hovered = true;
                    }
                    if response.clicked() {
                        clicked[i] = true;
                    }
                }
                if clicked[0] {
                    std::process::exit(0);
                }
                if clicked[1] {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                }
                if clicked[2] {
                    let maximized = ctx.input(|i| i.viewport().maximized).unwrap_or(false);
                    ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!maximized));
                }
                // Make the rest of the top bar draggable (except over the buttons)
                let drag_start = x_start + 3.0 * (circle_radius * 2.0 + spacing) + 10.0;
                let available = (ui.available_width() - (drag_start - ui.min_rect().left())).max(0.0);
                let (_rect, response) = ui.allocate_exact_size(egui::vec2(available, 28.0), egui::Sense::click_and_drag());
                if response.is_pointer_button_down_on() && !hovered {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(Color32::from_rgb(0xC0, 0xFF, 0xC0));
            ui.visuals_mut().extreme_bg_color = Color32::BLACK;
            ui.visuals_mut().code_bg_color = Color32::BLACK;
            ui.style_mut().override_text_style = Some(TextStyle::Monospace);

            egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                // Show the banner as the first line
                ui.label(egui::RichText::new("┌──(user@nInjaOS)-[~]").monospace().size(20.0).color(Color32::from_rgb(0xE9, 0x3E, 0xFF)));
                // Show all output lines
                for line in &self.output {
                    ui.label(egui::RichText::new(line).monospace().size(20.0).color(Color32::from_rgb(0xE9, 0x3E, 0xFF)));
                }
                // Prompt and input always as the last line
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("└─$ ").monospace().size(20.0).color(Color32::from_rgb(0xE9, 0x3E, 0xFF)));
                    let input_width = (self.input.len().max(1) as f32) * 12.0 + 10.0; // estimate width
                    let response = ui.add_sized([
                        input_width,
                        28.0
                    ],
                        TextEdit::singleline(&mut self.input)
                            .desired_width(input_width)
                            .font(TextStyle::Monospace)
                            .background_color(Color32::BLACK)
                            .text_color(Color32::from_rgb(0xC0, 0xFF, 0xC0))
                            .margin(egui::vec2(0.0, 6.0))
                            .frame(false)
                            .font(egui::FontId::monospace(20.0))
                    );
                    // Handle Enter key and always keep input focused
                    let enter_pressed = ui.input(|i| i.key_pressed(egui::Key::Enter));
                    if enter_pressed && !self.input.trim().is_empty() {
                        let command = self.input.trim().to_string();
                        self.output.push_back(format!("└─$ {}", command));
                        match command.as_str() {
                            "ls" => {
                                self.output.push_back("Desktop  Documents  Downloads  Pictures  Music".to_string());
                            }
                            "clear" => {
                                self.output.clear();
                            }
                            "sudo" | "apt" => {
                                self.output.push_back("This is a simulation. Use real Linux for full power!".to_string());
                            }
                            _ => {
                                let cmd_output = if cfg!(target_os = "windows") {
                                    Command::new("cmd")
                                        .args(["/C", &command])
                                        .output()
                                } else {
                                    Command::new("sh")
                                        .arg("-c")
                                        .arg(&command)
                                        .output()
                                };
                                match cmd_output {
                                    Ok(output) => {
                                        if !output.stdout.is_empty() {
                                            self.output.push_back(String::from_utf8_lossy(&output.stdout).to_string());
                                        }
                                        if !output.stderr.is_empty() {
                                            self.output.push_back(String::from_utf8_lossy(&output.stderr).to_string());
                                        }
                                    }
                                    Err(_) => self.output.push_back("Command failed to execute.".to_string()),
                                }
                            }
                        }
                        self.input.clear();
                        // Refocus input
                        ui.memory_mut(|mem| mem.request_focus(response.id));
                    } else if !response.has_focus() {
                        // Always keep input focused
                        ui.memory_mut(|mem| mem.request_focus(response.id));
                    }
                });
            });
        });
    }
}
