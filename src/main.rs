use eframe::{egui, App, Frame};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([210.0, 310.0])
            .with_min_inner_size([210.0, 310.0])
            .with_max_inner_size([210.0, 310.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Rustulator",
        options,
        Box::new(|_cc| Box::<CalculatorApp>::default()),
    )
}


#[derive(Default)]
struct CalculatorApp {
    expression: String,
    result: String,
}

impl App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| { 
                ui.text_edit_singleline(&mut self.expression);
                ui.label(
                    egui::RichText::new(&self.result)
                        .size(28.0)
                        .strong(),
                );

                ui.separator();

                let button = |ui: &mut egui::Ui, label: &str| {
                    let color = match label {
                        "C" => egui::Color32::from_rgb(220, 0, 0),     // Red
                        "=" => egui::Color32::from_rgb(0, 150, 0),     // Green
                        "+" | "-" | "×" | "÷" => egui::Color32::from_rgb(255, 165, 0), // Orange
                        _ => ui.visuals().widgets.inactive.bg_fill,    // Default
                    };
                
                    let text_color = egui::Color32::WHITE;
                
                    let button = egui::Button::new(
                        egui::RichText::new(label).color(text_color).size(18.0).strong()
                    )
                    .min_size(egui::vec2(44.0, 44.0))
                    .fill(color)
                    .rounding(egui::Rounding::same(6.0)); // rounded corners
                
                    if ui.add(button).clicked() {
                        Some(label.to_string())
                    } else {
                        None
                    }
                };
                
                let rows = vec![
                    vec!["C", "+/-", "%", "÷"],
                    vec!["7", "8", "9", "×"],
                    vec!["4", "5", "6", "-"],
                    vec!["1", "2", "3", "+"],
                    vec!["x²", "0", ".", "="],
                ];

                for row in rows {
                    ui.horizontal(|ui| {
                        for label in row {
                            if let Some(input) = button(ui, label) {
                                self.handle_input(input);
                            }
                        }
                    });
                }
            });
        });
    }
}

impl CalculatorApp {
    fn handle_input(&mut self, input: String) {
        match input.as_str() {
            "C" => {
                self.expression.clear();
                self.result.clear();
            }
            "=" => {
                match evaluate_expression(&self.expression) {
                    Ok(res) => self.result = format!("{}", res),
                    Err(e) => self.result = format!("Error: {}", e),
                }
            }
            "+/-" => {
                if self.expression.starts_with('-') {
                    self.expression.remove(0);
                } else if !self.expression.is_empty() {
                    self.expression = format!("-{}", self.expression);
                }
            }
            "%" => {
                if let Ok(val) = evaluate_expression(&self.expression) {
                    self.result = format!("{}", val / 100.0);
                    self.expression = self.result.clone();
                }
            }
            "x²" => {
                if let Ok(val) = evaluate_expression(&self.expression) {
                    self.result = format!("{}", val * val);
                    self.expression = self.result.clone();
                }
            }
            "." => {
                if !self.expression.ends_with('.') && !self.expression.contains('.') {
                    self.expression.push('.');
                }
            }
            _ => {
                self.expression.push_str(&input);
            }
        }
        
    }
}

fn evaluate_expression(expr: &str) -> Result<f64, String> {
    let expr = expr.replace("×", "*").replace("÷", "/");
    meval::eval_str(expr).map_err(|e| format!("{}", e))
}
