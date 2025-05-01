use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

use crate::config::DEFAULT_LANGUAGE;

pub static CURRENT_LANGUAGE: Lazy<RwLock<String>> = Lazy::new(|| {
    RwLock::new(DEFAULT_LANGUAGE.to_string())
});

pub static CURRENT_TRANSLATIONS: Lazy<RwLock<HashMap<&'static str, &'static str>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

pub fn set_language(locale: &str) {
    let translations = crate::lang::TRANSLATION_REGISTRY
        .get(locale)
        .unwrap_or_else(|| crate::lang::TRANSLATION_REGISTRY.get(DEFAULT_LANGUAGE).unwrap());

    {
        let mut guard = CURRENT_TRANSLATIONS.write().unwrap();
        *guard = (**translations).clone();
    }

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

/// Load translations dynamically from lang/{locale}.{ext}
pub fn load_translations(locale: &str) -> HashMap<&'static str, &'static str> {
    let base_path = format!("lang/{}", locale);
    let extensions = ["json", "yaml", "yml", "toml"];

    for ext in &extensions {
        let path = format!("{}.{}", base_path, ext);

        if let Ok(content) = std::fs::read_to_string(&path) {
            return parse_translation_content(ext, &content);
        }
    }

    panic!("Translation file for locale '{}' not found!", locale);
}

fn parse_translation_content(ext: &str, content: &str) -> HashMap<&'static str, &'static str> {
    match ext {
        "json" => {
            let parsed: HashMap<String, String> = serde_json::from_str(content).expect("Invalid JSON format");
            parsed.into_iter()
                .map(|(k, v)| (Box::leak(k.into_boxed_str()), Box::leak(v.into_boxed_str())))
                .collect()
        }
        "yaml" | "yml" => {
            let parsed: HashMap<String, String> = serde_yaml::from_str(content).expect("Invalid YAML format");
            parsed.into_iter()
                .map(|(k, v)| (Box::leak(k.into_boxed_str()), Box::leak(v.into_boxed_str())))
                .collect()
        }
        "toml" => {
            let parsed: HashMap<String, String> = toml::from_str(content).expect("Invalid TOML format");
            parsed.into_iter()
                .map(|(k, v)| (Box::leak(k.into_boxed_str()), Box::leak(v.into_boxed_str())))
                .collect()
        }
        _ => panic!("Unsupported file extension: '{}'", ext),
    }
}
