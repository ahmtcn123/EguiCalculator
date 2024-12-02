#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, vec2, Button, Label, Layout, RichText, Vec2};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([340.0, 410.0])
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "Egui Calculator",
        options,
        Box::new(|cc| Ok(Box::new(NumberButtonsApp::default()))),
    )
}

struct NumberButtonsApp {
    input_text: String,
    rendering_result: bool,
}

impl Default for NumberButtonsApp {
    fn default() -> Self {
        Self {
            input_text: String::new(),
            rendering_result: false,
        }
    }
}

impl eframe::App for NumberButtonsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                Layout::centered_and_justified(egui::Direction::BottomUp),
                |ui| {
                    // Disabled input field to display pressed buttons
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.add_enabled_ui(false, |ui| {
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.input_text)
                                        .min_size(vec2(ui.available_size_before_wrap().x, 50_f32))
                                        .hint_text("0")
                                        .font(egui::TextStyle::Heading.resolve(&ui.style())),
                                )
                            });
                        });

                        let button_values = vec![
                            vec!["C", "←", "%", "÷"],
                            vec!["7", "8", "9", "*"],
                            vec!["4", "5", "6", "-"],
                            vec!["1", "2", "3", "+"],
                            vec!["", "0", ",", "="],
                        ];

                        // Generate buttons in columns dynamically
                        for row in button_values.iter() {
                            ui.columns(4, |columns| {
                                for (i, value) in row.iter().enumerate() {
                                    let button = Button::new(RichText::new(*value).size(20.0));
                                    if value.is_empty() {
                                        columns[i].add_enabled_ui(false, |ui| {
                                            ui.add_sized(Vec2::new(75.0, 65.0), button);
                                        });
                                        continue;
                                    }

                                    let clicked = columns[i].add_sized(Vec2::new(75.0, 65.0), button).clicked();

                                    if clicked {
                                        // Handle button press logic
                                        self.handle_button_press(value);
                                    }
                                }
                            });
                        }
                    });
                },
            );
        });
    }
}

impl NumberButtonsApp {
    fn handle_button_press(&mut self, value: &str) {
        if self.rendering_result {
            self.input_text.clear();
            self.rendering_result = false;
        }

        match value {
            "C" => self.input_text.clear(),
            "←" => {
                self.input_text.pop();
            }
            "%" => self.input_text.push('%'),
            "÷" => self.input_text.push('/'), // Changed for proper division symbol
            "*" => self.input_text.push('*'),
            "-" => self.input_text.push('-'),
            "+" => self.input_text.push('+'),
            "=" => {
                // Evaluate the expression and display the result
                self.rendering_result = true;
                match meval::eval_str(&self.input_text) {
                    Ok(result) => {
                        self.input_text = result.to_string(); // Display result
                    }
                    Err(_) => {
                        self.input_text = "Error".to_string(); // Display error if evaluation fails
                    }
                }
            }
            "," => self.input_text.push('.'),
            "0" => {
                if self.input_text.is_empty() {
                    self.input_text.push('0');
                } else if self.input_text != "0" {
                    self.input_text.push('0');
                }
            }
            _ => self.input_text.push_str(value),
        }
    }
}
