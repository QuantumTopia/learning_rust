use std::io;
mod fibonacci;
mod generic_sort;
mod examples;
mod utils;
mod configuration;
use configuration::configuration as config;

fn main() {
    loop {
        println!("Welcome to the learning Rust project!");
        println!("Please select which module you would like to access:");
        let options = config::get_possible_options();
        
        for option in &options
        {
            println!("{}) {}", option.index, option.name);
        }

        module_selection(options);
    }
}

fn module_selection(options: Vec<config::Option>)
{
    let mut module_index = String::new();
    let _ = io::stdin().read_line(&mut module_index);
    let numerical_module_index = module_index.trim().parse::<u8>().unwrap();

    for option in options
    {
        if option.index == numerical_module_index
        {
            (option.option_entry)();
        }
    }
}
