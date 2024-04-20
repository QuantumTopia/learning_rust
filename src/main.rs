use std::io;
mod fibonacci;
mod utils;
mod configuration;
use fibonacci::fibonacci as fib;

fn main() {
    loop {
        println!("Welcome to the learning Rust project!");
        println!("Please select which module you would like to access:");
        println!("1) Fibonacci");
        println!("2) Generic sorting");

        let (module_index, module_name): (i32, String) =  module_selection();
        println!("Selected module: {}) {}", module_index, module_name);

        match module_index {
            1 => fib::run_fibonacci(),
            _ => println!("Apologies, this module is unimplemented")
        };
    }
}

fn module_selection() -> (i32, String) {
    let mut module_index = String::new();
    let _ = io::stdin().read_line(&mut module_index);
    let numerical_module_index = module_index.trim().parse::<i32>().unwrap();
    let module_name = match numerical_module_index {
        1 => "Fibonacci".to_string(),
        2 => "Generic sorting".to_string(),
        _ => "Error".to_string(),
    };
    (numerical_module_index, module_name)
}
