use eframe::App;
use egui::CentralPanel;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My GUI App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Brendan".to_owned(),
            age: 0,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });

            if ui.button("Click each year").clicked() {
                self.age += 1;
            }

            ui.label(format!("your name: {}, your age: {}", self.name, self.age));
        });
    }
}
