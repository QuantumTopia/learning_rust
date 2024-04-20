pub fn mod_print(message: &str) {
    println!("{}{}", crate::configuration::configuration::MODULE_SEPARATOR, message);
}