#[macro_export]
macro_rules! define_translations {
    ( [ $( ($key:expr, $value:expr) ),* $(,)? ] ) => {
        use once_cell::sync::Lazy;
        use std::collections::HashMap;

        pub static TRANSLATIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
            HashMap::from([
                $( ($key, $value) ),*
            ])
        });
    };
}