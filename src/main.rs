use eframe::{egui, App, Frame};

// Enum to map and style operator buttons
#[derive(Debug, Clone, Copy)]
enum CalculatorOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl CalculatorOperator {
    fn from_label(label: &str) -> Option<Self> {
        match label {
            "+" => Some(CalculatorOperator::Add),
            "-" => Some(CalculatorOperator::Subtract),
            "×" => Some(CalculatorOperator::Multiply),
            "÷" => Some(CalculatorOperator::Divide),
            _ => None,
        }
    }
}

// Main application state
#[derive(Default)]
struct CalculatorApp {
    expression: String,
    result: String,
}

impl App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // Input box
                ui.text_edit_singleline(&mut self.expression);
                // Result display
                ui.label(
                    egui::RichText::new(&self.result)
                        .size(28.0)
                        .strong(),
                );
                ui.separator();

                // Closure to render buttons and return a String on click
                let button = |ui: &mut egui::Ui, label: &str| -> Option<String> {
                    let bg = match label {
                        "C" => egui::Color32::from_rgb(220, 0, 0),
                        "=" => egui::Color32::from_rgb(0, 150, 0),
                        "+" | "-" | "×" | "÷" => egui::Color32::from_rgb(255, 165, 0),
                        _ => ui.visuals().widgets.inactive.bg_fill,
                    };
                    let txt = egui::Color32::WHITE;
                    let btn = egui::Button::new(
                        egui::RichText::new(label).color(txt).size(18.0).strong(),
                    )
                    .min_size(egui::vec2(44.0, 44.0))
                    .fill(bg)
                    .rounding(egui::Rounding::same(6.0));

                    if ui.add(btn).clicked() {
                        Some(label.to_string())
                    } else {
                        None
                    }
                };

                // Layout of buttons
                let rows = vec![
                    vec!["C", "+/-", "%", "÷"],
                    vec!["7", "8", "9", "×"],
                    vec!["4", "5", "6", "-"],
                    vec!["1", "2", "3", "+"],
                    vec!["x²", "0", ".", "="],
                ];

                for row in rows {
                    ui.horizontal(|ui| {
                        for &label in &row {
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
                // Evaluate the full expression on '='
                match evaluate_expression(&self.expression) {
                    Ok(val) => self.result = val.to_string(),
                    Err(err) => self.result = err,
                }
            }
            "+/-" => {
                if self.expression.starts_with('-') {
                    self.expression.remove(0);
                } else if !self.expression.is_empty() {
                    self.expression.insert_str(0, "-");
                }
            }
            "%" => {
                if let Ok(val) = evaluate_expression(&self.expression) {
                    let pct = val / 100.0;
                    self.result = pct.to_string();
                    self.expression = pct.to_string();
                }
            }
            "x²" => {
                if let Ok(val) = evaluate_expression(&self.expression) {
                    let sq = val * val;
                    self.result = sq.to_string();
                    self.expression = sq.to_string();
                }
            }
            "." => {
                // Prevent multiple decimals in a number segment
                if !self.expression.ends_with('.') {
                    self.expression.push('.');
                }
            }
            op if CalculatorOperator::from_label(op).is_some() => {
                // Prevent consecutive operators
                if let Some(last) = self.expression.chars().last() {
                    if "+-×÷".contains(last) {
                        self.expression.pop();
                    }
                }
                self.expression.push_str(op);
            }
            num => {
                // Append numbers
                self.expression.push_str(num);
            }
        }
    }
}

// Evaluate the arithmetic expression string
fn evaluate_expression(expr: &str) -> Result<f64, String> {
    let clean = expr.replace('×', "*").replace('÷', "/");
    meval::eval_str(&clean).map_err(|e| format!("{}", e))
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([240.0, 360.0])
            .with_min_inner_size([240.0, 360.0])
            .with_resizable(true),
        ..Default::default()
    };
    eframe::run_native(
        "Rustulator",
        options,
        Box::new(|_cc| Box::<CalculatorApp>::default()),
    )
}
