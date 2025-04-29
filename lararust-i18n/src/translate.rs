use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

use crate::config::DEFAULT_LANGUAGE;

// Current active language code (e.g., "en", "es")
pub static CURRENT_LANGUAGE: Lazy<RwLock<String>> = Lazy::new(|| {
    RwLock::new(DEFAULT_LANGUAGE.to_string())
});

// Current translations map (dynamic, runtime-swappable)
pub static CURRENT_TRANSLATIONS: Lazy<RwLock<HashMap<&'static str, &'static str>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

pub fn set_language(locale: &str) {
    let translations = crate::lang::TRANSLATION_REGISTRY
        .get(locale)
        .unwrap_or_else(|| crate::lang::TRANSLATION_REGISTRY.get(DEFAULT_LANGUAGE).unwrap());
    // Update translations atomically
    {
        let mut guard = CURRENT_TRANSLATIONS.write().unwrap();

        let _map = &translations;
        *guard = (**translations).clone(); 
    }

    // Update current language code
    {
        let mut current = CURRENT_LANGUAGE.write().unwrap();
        *current = locale.to_string();
    }
}

pub fn __(key: &str, replacements: &[(&str, &str)]) -> String {
    let translations = CURRENT_TRANSLATIONS.read().unwrap();

    let template = translations.get(key).copied().unwrap_or("");

    let mut result = template.to_string();
    for (placeholder, value) in replacements {
        let pattern = format!(":{}", placeholder);
        result = result.replace(&pattern, value);
    }

    result
}

pub fn __simple(key: &str) -> &'static str {
    let translations = CURRENT_TRANSLATIONS.read().unwrap();
    translations.get(key).copied().unwrap_or("")
}

/// Load translations from a JSON file dynamically.
/// Assumes `src/lang/{locale}.json` exists at build-time or runtime.
pub fn load_json_translations(locale: &str) -> HashMap<&'static str, &'static str> {
    let path = format!("src/lang/{}.json", locale);
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Missing or unreadable translation file for locale '{}'", locale));

    let parsed: HashMap<String, String> = serde_json::from_str(&content)
        .unwrap_or_else(|_| panic!("Invalid JSON format in translation file for locale '{}'", locale));

    parsed.into_iter()
        .map(|(k, v)| (Box::leak(k.into_boxed_str()), Box::leak(v.into_boxed_str())))
        .collect()
}