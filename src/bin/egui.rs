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
    age: u32,
    name: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            age: 0,
            name: "Brendan".to_owned(),
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.label(format!("your name: {}, your age: {}", self.name, self.age));
        });
    }
}
