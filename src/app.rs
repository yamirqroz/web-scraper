use crate::models::{Product, StoreManager};
use crate::ui::{ResultsTab, SearchTab, StoreTab};
use crate::utils::FileManager;
use eframe::egui;

pub struct ScrapingApp {
    store_manager: StoreManager,
    file_manager: FileManager,
    search_tab: SearchTab,
    store_tab: StoreTab,
    results_tab: ResultsTab,
    current_tab: Tab,
    search_results: Option<Vec<Product>>,
}

enum Tab {
    Search,
    Stores,
    Results,
}

impl ScrapingApp {
    pub fn new() -> Self {
        let file_manager = FileManager::new();
        let store_manager = file_manager.load_stores();
        
        //let app_config = file_manager.load_app_config();
        
        // Por:
        let _app_config = file_manager.load_app_config();
        
        Self {
            store_manager,
            file_manager,
            search_tab: SearchTab::new(),
            store_tab: StoreTab::new(),
            results_tab: ResultsTab::new(),
            current_tab: Tab::Search,
            search_results: None,
        }
    }
}

impl eframe::App for ScrapingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Sistema de Scraping de Productos");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("🔄 Guardar").clicked() {
                        if let Err(e) = self.file_manager.save_stores(&self.store_manager) {
                            eprintln!("Error al guardar tiendas: {}", e);
                        }
                    }
                });
            });
            
            ui.separator();
            
            ui.horizontal(|ui| {
                if ui.selectable_label(matches!(self.current_tab, Tab::Search), "🔍 Búsqueda").clicked() {
                    self.current_tab = Tab::Search;
                }
                if ui.selectable_label(matches!(self.current_tab, Tab::Stores), "🏪 Tiendas").clicked() {
                    self.current_tab = Tab::Stores;
                }
                if ui.selectable_label(matches!(self.current_tab, Tab::Results), "📊 Resultados").clicked() {
                    self.current_tab = Tab::Results;
                }
            });
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                Tab::Search => {
                    if let Some(results) = self.search_tab.show(ui, &mut self.store_manager) {
                        self.search_results = Some(results);
                        self.current_tab = Tab::Results;
                    }
                },
                Tab::Stores => {
                    self.store_tab.show(ui, &mut self.store_manager);
                },
                Tab::Results => {
                    self.results_tab.show(ui, self.search_results.as_ref());
                },
            }
        });
        
        // Auto-guardar cambios en tiendas
        ctx.request_repaint_after(std::time::Duration::from_secs(5));
    }
}