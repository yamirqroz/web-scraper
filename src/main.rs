use eframe::{egui, run_native};
use egui::CentralPanel;
use tokio;
use reqwest;
use scraper::{Html, Selector};
use url::Url;

use std::sync::mpsc as std_mpsc;
use serde::{Deserialize, Serialize}; // Necesario para serializar/deserializar JSON
use std::fs; // Para leer/escribir archivos
use std::path::PathBuf; // Para manejar rutas de archivos

// --- NUEVAS ESTRUCTURAS PARA PERFILES DE SCRAPING ---

/// Estructura para definir qué datos buscar en una página.
/// Cada campo es un selector CSS para un elemento específico.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct DataSelectors {
    product_name: String, // Selector CSS para el nombre del producto
    product_price: String, // Selector CSS para el precio del producto
    product_image_url: String, // Selector CSS para la URL de la imagen del producto
    // Puedes añadir más campos según sea necesario (ej. "description", "rating", "seller", etc.)
}

impl Default for DataSelectors {
    fn default() -> Self {
        // Valores por defecto para el perfil de quotes.toscrape.com
        Self {
            product_name: "div.quote span.text".to_owned(),
            product_price: "div.quote small.author".to_owned(),
            product_image_url: "".to_owned(), // quotes.toscrape.com no tiene imágenes de productos, lo dejamos vacío
        }
    }
}

/// Estructura que representa un perfil de scraping para un sitio web específico.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ScrapingProfile {
    id: String, // Un ID único para el perfil (ej. "quotes-toscrape-com")
    name: String, // Nombre amigable (ej. "Quotes to Scrape")
    base_url: String, // URL base del sitio (ej. "http://quotes.toscrape.com")
    selectors: DataSelectors, // Los selectores CSS para extraer datos
}

impl Default for ScrapingProfile {
    fn default() -> Self {
        // Perfil por defecto para quotes.toscrape.com
        Self {
            id: "quotes-toscrape-com".to_owned(),
            name: "Quotes to Scrape".to_owned(),
            base_url: "http://quotes.toscrape.com/".to_owned(),
            selectors: DataSelectors::default(),
        }
    }
}

// --- FUNCIONES PARA GESTIONAR PERFILES ---

// Función para obtener la ruta del archivo de configuración de perfiles
fn get_profiles_file_path() -> PathBuf {
    let mut path = eframe::storage_dir("my_app")
        .unwrap_or_else(|| PathBuf::from("."))
        .join("web_scraper_profiles.json");
    // Asegurarse de que el directorio existe (aunque storage_dir() ya lo crea en muchos casos)
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    path
}

// Función para cargar perfiles desde un archivo JSON
fn load_profiles() -> Vec<ScrapingProfile> {
    let path = get_profiles_file_path();
    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(data) => match serde_json::from_str(&data) {
                Ok(profiles) => {
                    eprintln!("Perfiles cargados desde: {:?}", path);
                    profiles
                },
                Err(e) => {
                    eprintln!("Error al deserializar perfiles de JSON: {}. Se cargarán perfiles por defecto.", e);
                    vec![ScrapingProfile::default()]
                }
            },
            Err(e) => {
                eprintln!("Error al leer archivo de perfiles: {}. Se cargarán perfiles por defecto.", e);
                vec![ScrapingProfile::default()]
            }
        }
    } else {
        eprintln!("Archivo de perfiles no encontrado en {:?}. Creando perfil por defecto.", path);
        // Si no existe, creamos un perfil por defecto y lo guardamos
        let default_profile = ScrapingProfile::default();
        let profiles = vec![default_profile];
        save_profiles(&profiles); // Guarda el perfil por defecto
        profiles
    }
}

// Función para guardar perfiles a un archivo JSON
fn save_profiles(profiles: &[ScrapingProfile]) {
    let path = get_profiles_file_path();
    match serde_json::to_string_pretty(profiles) {
        Ok(json) => {
            match fs::write(&path, json) {
                Ok(_) => eprintln!("Perfiles guardados en: {:?}", path),
                Err(e) => eprintln!("Error al escribir perfiles en archivo: {}", e),
            }
        },
        Err(e) => eprintln!("Error al serializar perfiles a JSON: {}", e),
    }
}


// --- FIN NUEVAS ESTRUCTURAS Y FUNCIONES ---


/// Estructura para almacenar los datos scrapeados (ahora más genérica).
#[derive(Debug, Clone)]
struct ScrapedData {
    product_name: String,   // Nombre del producto (o texto de cita)
    product_price: String,  // Precio (o autor de cita)
    product_image_url: String, // URL de la imagen (o vacía si no aplica)
    scraped_url: String,
}

/// Función asíncrona que realiza el scraping de una URL dada,
/// usando los selectores del perfil activo.
async fn perform_scraping(url: String, selectors: DataSelectors) -> Result<ScrapedData, String> {
    let parsed_url = match Url::parse(&url) {
        Ok(u) => u,
        Err(e) => return Err(format!("Error: La URL no es válida: {}", e)),
    };

    let response = match reqwest::get(parsed_url.as_str()).await {
        Ok(res) => res,
        Err(e) => return Err(format!("Error de red o conexión al intentar acceder a {}: {}", url, e)),
    };

    if !response.status().is_success() {
        return Err(format!("Error: La página {} devolvió un estado HTTP no exitoso: {}", url, response.status()));
    }

    let body = match response.text().await {
        Ok(text) => text,
        Err(e) => return Err(format!("Error al leer el contenido de la página {}: {}", url, e)),
    };

    let document = Html::parse_document(&body);

    // Usamos los selectores del perfil cargado
    let product_name_selector = match Selector::parse(&selectors.product_name) {
        Ok(s) => s,
        Err(_) => return Err(format!("Error interno: Selector de nombre de producto inválido: '{}'", selectors.product_name)),
    };

    let product_price_selector = match Selector::parse(&selectors.product_price) {
        Ok(s) => s,
        Err(_) => return Err(format!("Error interno: Selector de precio de producto inválido: '{}'", selectors.product_price)),
    };

    // El selector de imagen es opcional, solo lo usamos si no está vacío
    let product_image_url_selector = if !selectors.product_image_url.is_empty() {
        match Selector::parse(&selectors.product_image_url) {
            Ok(s) => Some(s),
            Err(_) => return Err(format!("Error interno: Selector de URL de imagen inválido: '{}'", selectors.product_image_url)),
        }
    } else {
        None
    };

    let product_name = document.select(&product_name_selector)
        .next()
        .map(|element| element.text().collect::<String>().trim().to_owned())
        .unwrap_or_else(|| "Nombre de producto no encontrado. (Revisar selector)".to_owned());

    let product_price = document.select(&product_price_selector)
        .next()
        .map(|element| element.text().collect::<String>().trim().to_owned())
        .unwrap_or_else(|| "Precio de producto no encontrado. (Revisar selector)".to_owned());

    let product_image_url = if let Some(selector) = product_image_url_selector {
        document.select(&selector)
            .next()
            .and_then(|element| element.value().attr("src")) // Asume que la URL está en el atributo 'src'
            .map(|s| s.to_owned())
            .unwrap_or_else(|| "URL de imagen no encontrada.".to_owned())
    } else {
        "No se configuró selector de imagen.".to_owned()
    };


    Ok(ScrapedData {
        product_name,
        product_price,
        product_image_url,
        scraped_url: url,
    })
}

type ScrapeResult = Result<ScrapedData, String>;

struct WebScraperApp {
    url_input: String,
    result_text: String,
    is_scraping: bool,

    sender: std_mpsc::Sender<ScrapeResult>,
    receiver: std_mpsc::Receiver<ScrapeResult>,

    // --- NUEVOS CAMPOS PARA LA GESTIÓN DE PERFILES ---
    profiles: Vec<ScrapingProfile>, // Lista de todos los perfiles cargados/creados
    active_profile_index: usize, // Índice del perfil activo en la lista `profiles`
    // --- FIN NUEVOS CAMPOS ---
}

impl WebScraperApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (sender, receiver) = std_mpsc::channel();
        let profiles = load_profiles(); // Carga los perfiles al inicio
        let initial_url = profiles.first().map(|p| p.base_url.clone()).unwrap_or_else(|| "http://quotes.toscrape.com/".to_owned());


        Self {
            url_input: initial_url,
            result_text: "Selecciona un perfil y una URL, luego haz clic en 'Scrapear'.".to_owned(),
            is_scraping: false,
            sender,
            receiver,
            profiles,
            active_profile_index: 0, // Por defecto, el primer perfil es el activo
        }
    }
}

impl eframe::App for WebScraperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("🚀 Web Scraper Rust Edition 🚀");
            ui.add_space(15.0);

            // --- SECCIÓN DE GESTIÓN DE PERFILES (Dropdown) ---
            ui.horizontal(|ui| {
                ui.label("Perfil Activo:");
                let current_profile_name = self.profiles
                    .get(self.active_profile_index)
                    .map(|p| p.name.clone())
                    .unwrap_or_else(|| "Ninguno".to_owned());

                egui::ComboBox::from_label("Seleccionar Perfil")
                    .selected_text(current_profile_name)
                    .show_ui(ui, |ui| {
                        for (i, profile) in self.profiles.iter().enumerate() {
                            if ui.selectable_value(&mut self.active_profile_index, i, &profile.name).clicked() {
                                // Cuando se selecciona un nuevo perfil, actualiza la URL de entrada.
                                self.url_input = profile.base_url.clone();
                            }
                        }
                    });
            });
            ui.add_space(10.0);
            // --- FIN SECCIÓN DE PERFILES ---


            ui.horizontal(|ui| {
                ui.label("URL a Scrapear:");
                ui.text_edit_singleline(&mut self.url_input);
            });
            ui.add_space(10.0);

            let button_text = if self.is_scraping {
                "Scrapeando... Por favor, espera."
            } else {
                "✨ Scrapear y Buscar Información ✨"
            };

            let button_response = ui.add_enabled_ui(!self.is_scraping, |ui| {
                ui.button(button_text)
            }).inner;

            if button_response.clicked() {
                let url_to_scrape = self.url_input.clone();
                let sender_clone = self.sender.clone();
                // Asegúrate de obtener los selectores del perfil ACTIVO
                let active_selectors = self.profiles
                    .get(self.active_profile_index)
                    .map(|p| p.selectors.clone())
                    .unwrap_or_default(); // Usa los selectores por defecto si no hay perfil activo

                self.is_scraping = true;
                self.result_text = format!("Iniciando scraping de {} usando perfil '{}'...", url_to_scrape, self.profiles[self.active_profile_index].name);
                ctx.request_repaint();

                tokio::spawn(async move {
                    // Pasa los selectores a la función de scraping
                    let result = perform_scraping(url_to_scrape, active_selectors).await;
                    let _ = sender_clone.send(result);
                });
            }

            ui.add_space(20.0);

            while let Ok(result) = self.receiver.try_recv() {
                match result {
                    Ok(scraped_data) => {
                        self.result_text = format!(
                            "✅ Scraping Exitoso!\n\nURL Scrapeada: {}\n\nNombre (o Cita): \n{}\n\nPrecio (o Autor): \n{}\n\nURL de Imagen: {}",
                            scraped_data.scraped_url,
                            scraped_data.product_name,
                            scraped_data.product_price,
                            scraped_data.product_image_url
                        );
                    },
                    Err(e) => {
                        self.result_text = format!("❌ Error en el scraping:\n{}", e);
                    }
                }
                self.is_scraping = false;
            }

            if self.is_scraping {
                ctx.request_repaint();
            }

            ui.label("📊 Resultados Obtenidos:");
            ui.group(|ui| {
                ui.add(egui::TextEdit::multiline(&mut self.result_text)
                    .desired_width(f32::INFINITY)
                    .desired_rows(15)
                    .interactive(false)
                );
            });
        });
    }
}

#[tokio::main]
async fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_min_inner_size([500.0, 400.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon.png"))
                    .expect("Failed to load icon from assets/icon.png. Make sure the file exists and is a valid PNG."),
            ),
        ..Default::default()
    };

    run_native(
        "Web Scraper de Ofertas en Rust",
        options,
        Box::new(|cc| Box::new(WebScraperApp::new(cc))),
    )
}