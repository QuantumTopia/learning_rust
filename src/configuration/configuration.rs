pub static MODULE_SEPARATOR: &str = ">\t";


// // Import the lazy_static crate for lazy initialization
// extern crate lazy_static;

// use lazy_static::lazy_static;

// // Define your configuration struct
// struct Configuration {
//     // Define configuration fields here
//     fibonacci_module_name: String,
//     // Add more configuration fields as needed
//     generic_sort_module_name: String,
//     module_separator: String,
// }

// impl Configuration {
//     // Initialize the configuration
//     fn new() -> Configuration {
//         // Load configuration from environment variables, files, etc.
//         Configuration {
//             fibonacci_module_name: String::from("Fibonacci"),
//             generic_sort_module_name: String::from("Generic sort"),
//             module_separator: String::from(">\t"),
//         }
//     }
// }

// // Define a lazy-loaded static variable to hold the configuration
// lazy_static! {
//     static ref CONFIG: Configuration = Configuration::new();
// }

// // Function to access the configuration
// pub fn get_configuration() -> &'static Configuration {
//     &CONFIG
// }