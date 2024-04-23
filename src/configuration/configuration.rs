use crate::fibonacci::fibonacci::run_fibonacci as run_fib;
use crate::generic_sort::generic_sort::run_generic_sort as run_sort;
use crate::examples::examples::example_stack as run_examples;

pub static MODULE_SEPARATOR: &str = ">\t";

pub struct Option
{
    pub index: u8,
    pub name: String,
    pub option_entry: fn() -> (),
}

pub fn get_possible_options() -> Vec<Option>
{
    let mut options: Vec<Option> = Vec::new();
    options.push(Option
    {
        index: 1,
        name: String::from("Fibonacci"),
        option_entry: run_fib,
    });
    options.push(Option
    {
        index: 2,
        name: String::from("Generic Sort"),
        option_entry: run_sort,
    });
    options.push(Option
    {
        index: 3,
        name: String::from("Examples"),
        option_entry: run_examples,
    });
    options
}