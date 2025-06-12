use crate::models::{Product, StoreConfig};
use crate::scraping::selectors::SelectorHelper;
use scraper::{Html, Selector};

pub struct WebScraper;

impl WebScraper {
    pub fn new() -> Self {
        Self
    }

    /// Realiza scraping de múltiples productos en una página
    pub fn scrape_products(&self, url: &str, store_config: &StoreConfig) -> Result<Vec<Product>, String> {
        let html = self.fetch_html(url)?;
        let document = Html::parse_document(&html);
        let mut products = Vec::new();

        // Selector para encontrar todos los contenedores de productos
        let container_selector = match Selector::parse(&store_config.product_container_selector) {
            Ok(selector) => selector,
            Err(e) => return Err(format!("Error en selector de contenedor: {}", e)),
        };

        // Iterar sobre cada producto encontrado
        for product_element in document.select(&container_selector) {
            if let Some(product) = self.extract_product_data(&product_element, store_config, url) {
                products.push(product);
            }
        }

        Ok(products)
    }

    /// Realiza scraping de un solo producto
    pub fn scrape_single_product(&self, url: &str, store_config: &StoreConfig) -> Result<Option<Product>, String> {
        let html = self.fetch_html(url)?;
        let document = Html::parse_document(&html);

        if let Some(product) = self.extract_product_data(&document.root_element(), store_config, url) {
            Ok(Some(product))
        } else {
            Ok(None)
        }
    }

    /// Busca productos usando el término de búsqueda
    pub fn search_products(&self, query: &str, store_config: &StoreConfig) -> Result<Vec<Product>, String> {
        let search_url = store_config.build_search_url(query);
        self.scrape_products(&search_url, store_config)
    }

    /// Obtiene el HTML de una URL
    fn fetch_html(&self, url: &str) -> Result<String, String> {
        match reqwest::blocking::get(url) {
            Ok(response) => {
                if response.status().is_success() {
                    match response.text() {
                        Ok(html) => Ok(html),
                        Err(e) => Err(format!("Error al leer el contenido: {}", e)),
                    }
                } else {
                    Err(format!("Error HTTP: {}", response.status()))
                }
            }
            Err(e) => Err(format!("Error de conexión: {}", e)),
        }
    }

    /// Extrae los datos de un producto desde un elemento HTML
    fn extract_product_data(
        &self,
        element: &scraper::ElementRef,
        store_config: &StoreConfig,
        base_url: &str,
    ) -> Option<Product> {
        let helper = SelectorHelper::new();

        // Extraer información básica
        let name = helper.extract_text(element, &store_config.name_selector)?;
        let price = helper.extract_text(element, &store_config.price_selector)?;
        
        // Extraer URL del producto
        let product_url = helper
            .extract_attribute(element, &store_config.link_selector, "href")
            .map(|url| self.resolve_url(base_url, &url))
            .unwrap_or_else(|| base_url.to_string());

        // Extraer imagen
        let image_url = helper
            .extract_attribute(element, &store_config.image_selector, "src")
            .map(|url| self.resolve_url(base_url, &url))
            .unwrap_or_default();

        // Extraer descripción si está configurada
        let description = if let Some(desc_selector) = &store_config.description_selector {
            helper.extract_text(element, desc_selector)
        } else {
            None
        };

        let mut product = Product::new(
            name,
            price,
            product_url,
            image_url,
            store_config.name.clone(),
        );

        if let Some(desc) = description {
            product = product.with_description(desc);
        }

        Some(product)
    }

    /// Resuelve URLs relativas a absolutas
    fn resolve_url(&self, base_url: &str, relative_url: &str) -> String {
        if relative_url.starts_with("http") {
            relative_url.to_string()
        } else if relative_url.starts_with("//") {
            format!("https:{}", relative_url)
        } else if relative_url.starts_with('/') {
            // Extraer el dominio base
            if let Ok(parsed_base) = reqwest::Url::parse(base_url) {
                format!("{}://{}{}", parsed_base.scheme(), parsed_base.host_str().unwrap_or(""), relative_url)
            } else {
                relative_url.to_string()
            }
        } else {
            format!("{}/{}", base_url.trim_end_matches('/'), relative_url)
        }
    }
}

impl Default for WebScraper {
    fn default() -> Self {
        Self::new()
    }
}