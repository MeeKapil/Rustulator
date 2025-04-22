use eframe::{egui, App, Frame};

// Main function that sets up and runs the application.
fn main() -> Result<(), eframe::Error> {
    // Configure the options for the native window.
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([210.0, 310.0]) // Set initial size of the window.
            .with_min_inner_size([210.0, 310.0]) // Set minimum size.
            .with_max_inner_size([210.0, 310.0]) // Set maximum size.
            .with_resizable(true), // Allow resizing the window.
        ..Default::default() // Use default values for other settings.
    };

    // Start the application with the given options.
    eframe::run_native(
        "Rustulator", // Title of the app.
        options, // Options for the window.
        Box::new(|_cc| Box::<CalculatorApp>::default()), // Instantiate the CalculatorApp.
    )
}

// Struct to represent the state of the calculator app.
#[derive(Default)]
struct CalculatorApp {
    expression: String, // Holds the current expression typed by the user.
    result: String, // Holds the result of the expression.
}

// Implementation of the App trait for CalculatorApp.
impl App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // Create the central panel for the UI.
        egui::CentralPanel::default().show(ctx, |ui| {
            // Vertically center all the UI elements.
            ui.vertical_centered(|ui| { 
                // Textbox for entering mathematical expressions.
                ui.text_edit_singleline(&mut self.expression);
                
                // Display the result in a larger, bold text.
                ui.label(
                    egui::RichText::new(&self.result)
                        .size(28.0)
                        .strong(),
                );

                // Add a separator line for visual separation.
                ui.separator();

                // Function to create buttons with specific labels and colors.
                let button = |ui: &mut egui::Ui, label: &str| {
                    let color = match label {
                        "C" => egui::Color32::from_rgb(220, 0, 0), // Red for Clear
                        "=" => egui::Color32::from_rgb(0, 150, 0), // Green for Equals
                        "+" | "-" | "×" | "÷" => egui::Color32::from_rgb(255, 165, 0), // Orange for operators
                        _ => ui.visuals().widgets.inactive.bg_fill, // Default color for other buttons
                    };
                
                    let text_color = egui::Color32::WHITE; // White text color.
                
                    // Create a button with a custom style and label.
                    let button = egui::Button::new(
                        egui::RichText::new(label).color(text_color).size(18.0).strong()
                    )
                    .min_size(egui::vec2(44.0, 44.0)) // Minimum button size.
                    .fill(color) // Set background color.
                    .rounding(egui::Rounding::same(6.0)); // Rounded corners for the button.
                
                    // If the button is clicked, return the label.
                    if ui.add(button).clicked() {
                        Some(label.to_string())
                    } else {
                        None
                    }
                };

                // Layout of the calculator buttons (rows of button labels).
                let rows = vec![
                    vec!["C", "+/-", "%", "÷"],
                    vec!["7", "8", "9", "×"],
                    vec!["4", "5", "6", "-"],
                    vec!["1", "2", "3", "+"],
                    vec!["x²", "0", ".", "="],
                ];

                // Iterate through the rows and create horizontal button groups.
                for row in rows {
                    ui.horizontal(|ui| {
                        for label in row {
                            // Handle button clicks.
                            if let Some(input) = button(ui, label) {
                                self.handle_input(input); // Process the input.
                            }
                        }
                    });
                }
            });
        });
    }
}

// Methods for the CalculatorApp.
impl CalculatorApp {
    // This method handles button presses and updates the expression or result.
    fn handle_input(&mut self, input: String) {
        match input.as_str() {
            "C" => {
                // Clear both the expression and result when 'C' is pressed.
                self.expression.clear();
                self.result.clear();
            }
            "=" => {
                // Evaluate the expression and show the result.
                match evaluate_expression(&self.expression) {
                    Ok(res) => self.result = format!("{}", res), // Format result as string.
                    Err(e) => self.result = format!("Error: {}", e), // Show error message if evaluation fails.
                }
            }
            "+/-" => {
                // Toggle the sign of the current expression (if it starts with '-').
                if self.expression.starts_with('-') {
                    self.expression.remove(0);
                } else if !self.expression.is_empty() {
                    self.expression = format!("-{}", self.expression);
                }
            }
            "%" => {
                // Convert the expression to a percentage.
                if let Ok(val) = evaluate_expression(&self.expression) {
                    self.result = format!("{}", val / 100.0); // Result is the value divided by 100.
                    self.expression = self.result.clone();
                }
            }
            "x²" => {
                // Square the result of the expression.
                if let Ok(val) = evaluate_expression(&self.expression) {
                    self.result = format!("{}", val * val); // Result is the value squared.
                    self.expression = self.result.clone();
                }
            }
            "." => {
                // Add a decimal point if not already present in the expression.
                if !self.expression.ends_with('.') && !self.expression.contains('.') {
                    self.expression.push('.');
                }
            }
            _ => {
                // Append the clicked button's label to the expression.
                self.expression.push_str(&input);
            }
        }
    }
}

// Function to evaluate the mathematical expression.
fn evaluate_expression(expr: &str) -> Result<f64, String> {
    // Replace "×" and "÷" with "*" and "/" for evaluation.
    let expr = expr.replace("×", "*").replace("÷", "/");
    
    // Use the `meval` library to evaluate the expression.
    meval::eval_str(expr).map_err(|e| format!("{}", e)) // Return the result or an error.
}
