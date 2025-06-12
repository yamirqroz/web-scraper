use crate::models::Product;
use crate::utils::FileManager;
use eframe::egui;

pub struct ResultsTab {
    selected_product: Option<usize>,
    file_manager: FileManager,
    show_save_message: bool,
    save_message: String,
    save_message_time: f32,
}

impl ResultsTab {
    pub fn new() -> Self {
        Self {
            selected_product: None,
            file_manager: FileManager::new(),
            show_save_message: false,
            save_message: String::new(),
            save_message_time: 0.0,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, products: Option<&Vec<Product>>) {
        ui.heading("📊 Resultados de Búsqueda");
        ui.separator();

        if let Some(products) = products {
            if products.is_empty() {
                ui.label("No hay resultados para mostrar.");
                return;
            }

            ui.horizontal(|ui| {
                if ui.button("💾 Guardar Resultados").clicked() {
                    match self.file_manager.save_search_results(products) {
                        Ok(_) => {
                            self.show_save_message = true;
                            self.save_message = "✅ Resultados guardados correctamente".to_string();
                            self.save_message_time = 3.0;
                        }
                        Err(e) => {
                            self.show_save_message = true;
                            self.save_message = format!("❌ Error al guardar: {}", e);
                            self.save_message_time = 5.0;
                        }
                    }
                }

                if self.show_save_message {
                    ui.label(&self.save_message);
                    self.save_message_time -= ui.ctx().input(|i| i.unstable_dt);
                    if self.save_message_time <= 0.0 {
                        self.show_save_message = false;
                    }
                }
            });

            ui.add_space(10.0);

            // Panel dividido: lista de productos a la izquierda, detalles a la derecha
            egui::SidePanel::left("products_list")
                .resizable(true)
                .default_width(300.0)
                .show_inside(ui, |ui| {
                    ui.heading("Productos");
                    ui.separator();

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for (i, product) in products.iter().enumerate() {
                            let is_selected = self.selected_product == Some(i);
                            let response = ui.selectable_label(
                                is_selected,
                                format!("{}\n{}", product.name, product.price),
                            );

                            if response.clicked() {
                                self.selected_product = Some(i);
                            }
                        }
                    });
                });

            // Panel de detalles del producto
            if let Some(selected_idx) = self.selected_product {
                if let Some(product) = products.get(selected_idx) {
                    ui.vertical(|ui| {
                        ui.heading(&product.name);
                        ui.label(format!("Precio: {}", product.price));
                        ui.label(format!("Tienda: {}", product.store_name));
                        
                        if let Some(desc) = &product.description {
                            ui.label(format!("Descripción: {}", desc));
                        }

                        ui.horizontal(|ui| {
                            if ui.button("🔗 Abrir enlace").clicked() {
                                if let Err(e) = open::that(&product.url) {
                                    eprintln!("Error al abrir URL: {}", e);
                                }
                            }

                            ui.hyperlink_to("Ver en tienda", &product.url);
                        });

                        ui.add_space(10.0);
                        
                        // Mostrar imagen si está disponible
                        if !product.image_url.is_empty() {
                            ui.label("Imagen del producto:");
                            ui.hyperlink_to("Ver imagen", &product.image_url);
                            // Nota: Para mostrar la imagen directamente, necesitarías
                            // implementar carga de imágenes con egui, lo cual requiere
                            // funcionalidades adicionales
                        }
                    });
                }
            } else {
                ui.label("Selecciona un producto para ver detalles");
            }
        } else {
            ui.label("Realiza una búsqueda para ver resultados.");
        }
    }
}

impl Default for ResultsTab {
    fn default() -> Self {
        Self::new()
    }
}