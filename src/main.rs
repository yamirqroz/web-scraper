mod app;
mod models;
mod scraping;
mod ui;
mod utils;

use app::ScrapingApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_title("Sistema de Scraping de Productos"),
        ..Default::default()
    };

    eframe::run_native(
        "Scraping System",
        options,
        Box::new(|_cc| Box::new(ScrapingApp::new())),
    )
}