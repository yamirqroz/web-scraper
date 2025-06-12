use scraper::{ElementRef, Selector};

pub struct SelectorHelper;

impl SelectorHelper {
    pub fn new() -> Self {
        Self
    }

    /// Extrae texto de un elemento usando un selector CSS
    pub fn extract_text(&self, element: &ElementRef, selector_str: &str) -> Option<String> {
        if selector_str.is_empty() {
            return None;
        }

        let selector = Selector::parse(selector_str).ok()?;
        
        if let Some(selected_element) = element.select(&selector).next() {
            let text = selected_element.text().collect::<String>().trim().to_string();
            if !text.is_empty() {
                Some(text)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Extrae un atributo de un elemento usando un selector CSS
    pub fn extract_attribute(&self, element: &ElementRef, selector_str: &str, attribute: &str) -> Option<String> {
        if selector_str.is_empty() {
            return None;
        }

        let selector = Selector::parse(selector_str).ok()?;
        
        if let Some(selected_element) = element.select(&selector).next() {
            selected_element.value().attr(attribute).map(|s| s.to_string())
        } else {
            None
        }
    }

    /// Extrae múltiples textos de elementos usando un selector CSS
    pub fn extract_multiple_texts(&self, element: &ElementRef, selector_str: &str) -> Vec<String> {
        if selector_str.is_empty() {
            return Vec::new();
        }

        let selector = match Selector::parse(selector_str) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        element
            .select(&selector)
            .map(|el| el.text().collect::<String>().trim().to_string())
            .filter(|text| !text.is_empty())
            .collect()
    }

    /// Valida si un selector CSS es válido
    pub fn validate_selector(&self, selector_str: &str) -> bool {
        if selector_str.is_empty() {
            return false;
        }
        Selector::parse(selector_str).is_ok()
    }

    /// Sugiere selectores comunes para elementos típicos
    pub fn suggest_selectors(&self, element_type: &str) -> Vec<String> {
        match element_type.to_lowercase().as_str() {
            "title" | "name" => vec![
                ".product-title".to_string(),
                ".product-name".to_string(),
                "h1".to_string(),
                "h2".to_string(),
                ".title".to_string(),
                "[data-testid='product-title']".to_string(),
            ],
            "price" => vec![
                ".price".to_string(),
                ".product-price".to_string(),
                ".current-price".to_string(),
                ".sale-price".to_string(),
                "[data-testid='price']".to_string(),
                ".price-current".to_string(),
            ],
            "image" => vec![
                ".product-image img".to_string(),
                ".main-image img".to_string(),
                "img.product-photo".to_string(),
                "[data-testid='product-image'] img".to_string(),
            ],
            "link" => vec![
                "a".to_string(),
                ".product-link".to_string(),
                "a.product-title".to_string(),
                "[data-testid='product-link']".to_string(),
            ],
            "description" => vec![
                ".product-description".to_string(),
                ".description".to_string(),
                ".product-summary".to_string(),
                "[data-testid='description']".to_string(),
            ],
            "container" => vec![
                ".product-item".to_string(),
                ".product-card".to_string(),
                ".product".to_string(),
                "[data-testid='product']".to_string(),
                ".search-result".to_string(),
            ],
            _ => Vec::new(),
        }
    }
}

impl Default for SelectorHelper {
    fn default() -> Self {
        Self::new()
    }
}