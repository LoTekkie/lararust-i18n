use lararust_i18n::{set_language, __};

fn main() {
    set_language("es");
    println!("{}", __("menu.quit", &[])); // "Salir"
    println!("{}", __("greeting.hello", &[("name", "Carlos")])); // "¡Hola, Carlos!"

    set_language("en");
    println!("{}", __("menu.quit", &[])); // "Quit"
    println!("{}", __("greeting.hello", &[("name", "Sid")])); // "Hello, Sid!"
}
