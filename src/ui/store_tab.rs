use crate::models::{StoreConfig, StoreManager};
use crate::scraping::selectors::SelectorHelper;
use eframe::egui;

pub struct StoreTab {
    new_store: StoreConfig,
    selected_store: Option<usize>,
    editing: bool,
    selector_helper: SelectorHelper,
    test_url: String,
    selector_type: String,
    show_suggestions: bool,
}

impl StoreTab {
    pub fn new() -> Self {
        Self {
            new_store: StoreConfig::default(),
            selected_store: None,
            editing: false,
            selector_helper: SelectorHelper::new(),
            test_url: String::new(),
            selector_type: "container".to_string(),
            show_suggestions: false,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, store_manager: &mut StoreManager) {
        ui.heading("🏪 Gestión de Tiendas");
        ui.separator();

        // Panel dividido: lista de tiendas a la izquierda, formulario a la derecha
        egui::SidePanel::left("stores_list")
            .resizable(true)
            .default_width(200.0)
            .show_inside(ui, |ui| {
                ui.heading("Tiendas");
                ui.separator();

                if ui.button("➕ Nueva Tienda").clicked() {
                    self.new_store = StoreConfig::default();
                    self.editing = false;
                    self.selected_store = None;
                }

                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (i, store) in store_manager.stores.iter().enumerate() {
                        let is_selected = self.selected_store == Some(i);
                        let response = ui.selectable_label(is_selected, &store.name);

                        if response.clicked() {
                            self.selected_store = Some(i);
                            self.new_store = store.clone();
                            self.editing = true;
                        }
                    }
                });
            });

        // Formulario de tienda
        ui.vertical(|ui| {
            ui.heading(if self.editing {
                format!("Editar Tienda: {}", self.new_store.name)
            } else {
                "Nueva Tienda".to_string()
            });
            ui.separator();

            // Campos básicos
            ui.horizontal(|ui| {
                ui.label("Nombre:");
                ui.text_edit_singleline(&mut self.new_store.name);
            });

            ui.horizontal(|ui| {
                ui.label("URL Base:");
                ui.text_edit_singleline(&mut self.new_store.base_url);
            });

            ui.horizontal(|ui| {
                ui.label("Patrón URL Búsqueda:");
                ui.text_edit_singleline(&mut self.new_store.search_url_pattern);
            });

            ui.collapsing("Ayuda sobre patrones de URL", |ui| {
                ui.label("Usa {base_url} para la URL base y {query} para el término de búsqueda.");
                ui.label("Ejemplo: {base_url}/search?q={query}");
            });

            ui.checkbox(&mut self.new_store.enabled, "Habilitada");

            ui.separator();
            ui.heading("Selectores CSS");

            // Selectores - Modificado para evitar préstamos múltiples
            // En lugar de llamar a un método que toma &mut self, trabajamos directamente con los campos
            
            // Contenedor de Producto
            ui.horizontal(|ui| {
                ui.label("Contenedor de Producto:");
                ui.text_edit_singleline(&mut self.new_store.product_container_selector);
                
                if ui.button("Sugerencias").clicked() {
                    self.show_suggestions = true;
                    self.selector_type = "container".to_string();
                }
            });
            
            // Nombre del Producto
            ui.horizontal(|ui| {
                ui.label("Nombre del Producto:");
                ui.text_edit_singleline(&mut self.new_store.name_selector);
                
                if ui.button("Sugerencias").clicked() {
                    self.show_suggestions = true;
                    self.selector_type = "title".to_string();
                }
            });
            
            // Precio
            ui.horizontal(|ui| {
                ui.label("Precio:");
                ui.text_edit_singleline(&mut self.new_store.price_selector);
                
                if ui.button("Sugerencias").clicked() {
                    self.show_suggestions = true;
                    self.selector_type = "price".to_string();
                }
            });
            
            // Imagen
            ui.horizontal(|ui| {
                ui.label("Imagen:");
                ui.text_edit_singleline(&mut self.new_store.image_selector);
                
                if ui.button("Sugerencias").clicked() {
                    self.show_suggestions = true;
                    self.selector_type = "image".to_string();
                }
            });
            
            // Enlace
            ui.horizontal(|ui| {
                ui.label("Enlace:");
                ui.text_edit_singleline(&mut self.new_store.link_selector);
                
                if ui.button("Sugerencias").clicked() {
                    self.show_suggestions = true;
                    self.selector_type = "link".to_string();
                }
            });
            
            // Selector de descripción (opcional)
            ui.horizontal(|ui| {
                ui.label("Descripción (opcional):");
                let mut has_description = self.new_store.description_selector.is_some();
                ui.checkbox(&mut has_description, "");
                
                if has_description {
                    let mut desc = self.new_store.description_selector.clone().unwrap_or_default();
                    ui.text_edit_singleline(&mut desc);
                    self.new_store.description_selector = Some(desc);
                    
                    if ui.button("Sugerencias").clicked() {
                        self.show_suggestions = true;
                        self.selector_type = "description".to_string();
                    }
                } else {
                    self.new_store.description_selector = None;
                }
            });

            // Herramienta de prueba de selectores
            ui.collapsing("Herramienta de prueba", |ui| {
                ui.horizontal(|ui| {
                    ui.label("URL para probar:");
                    ui.text_edit_singleline(&mut self.test_url);
                });
                
                if ui.button("Probar Selectores").clicked() {
                    // Aquí iría la lógica para probar los selectores
                    // Esto requeriría implementar una función que haga scraping
                    // y muestre los resultados en tiempo real
                }
            });

            // Mostrar sugerencias de selectores
            if self.show_suggestions {
                egui::Window::new("Sugerencias de Selectores")
                    .collapsible(false)
                    .show(ui.ctx(), |ui| {
                        ui.heading(format!("Sugerencias para {}", self.selector_type));
                        
                        let suggestions = self.selector_helper.suggest_selectors(&self.selector_type);
                        
                        for suggestion in suggestions {
                            if ui.button(&suggestion).clicked() {
                                match self.selector_type.as_str() {
                                    "container" => self.new_store.product_container_selector = suggestion.clone(),
                                    "title" => self.new_store.name_selector = suggestion.clone(),
                                    "price" => self.new_store.price_selector = suggestion.clone(),
                                    "image" => self.new_store.image_selector = suggestion.clone(),
                                    "link" => self.new_store.link_selector = suggestion.clone(),
                                    "description" => self.new_store.description_selector = Some(suggestion.clone()),
                                    _ => {}
                                }
                                self.show_suggestions = false;
                            }
                        }
                        
                        if ui.button("Cerrar").clicked() {
                            self.show_suggestions = false;
                        }
                    });
            }

            ui.separator();

            // Botones de acción
            ui.horizontal(|ui| {
                if ui.button(if self.editing { "💾 Actualizar" } else { "💾 Guardar" }).clicked() {
                    if self.new_store.is_valid() {
                        if self.editing {
                            if let Some(idx) = self.selected_store {
                                store_manager.stores[idx] = self.new_store.clone();
                            }
                        } else {
                            store_manager.add_store(self.new_store.clone());
                        }
                        
                        self.new_store = StoreConfig::default();
                        self.editing = false;
                        self.selected_store = None;
                    } else {
                        // Mostrar error
                    }
                }

                if self.editing {
                    if ui.button("🗑️ Eliminar").clicked() {
                        if let Some(idx) = self.selected_store {
                            store_manager.stores.remove(idx);
                            self.new_store = StoreConfig::default();
                            self.editing = false;
                            self.selected_store = None;
                        }
                    }
                }

                if ui.button("❌ Cancelar").clicked() {
                    self.new_store = StoreConfig::default();
                    self.editing = false;
                    self.selected_store = None;
                }
            });
        });
    }
}

impl Default for StoreTab {
    fn default() -> Self {
        Self::new()
    }
}