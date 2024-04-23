use std::io;

use crate::utils::print_utils::mod_print as mod_print;

pub fn run_fibonacci() {
    welcome_message();
    let mut selected_index = process_input();
    provide_result(selected_index);
    loop {
        mod_print("Would you like to know another number from the sequence? (Use -1 to return back to module selection)");
        selected_index = process_input();
        if selected_index == -1 { break; }
        provide_result(selected_index);
    }
    
}

fn welcome_message() -> () {
    mod_print("Welcome to the Fibonacci number machine!");
    mod_print("");
    mod_print("The beginning sequence looks like:");
    let numbers: [i32; 11] = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
    let indexes: [i32; 11] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    mod_print(format!("{}\t{}", "Numbers: ", string_joiner(numbers)).as_str());
    mod_print(format!("{}\t{}", "Indexes: ", string_joiner(indexes)).as_str());
    mod_print("");
    mod_print("Which Fibonacci number would you like to know?");
}

fn process_input() -> i32 {
    let mut fibo_index = String::new();
    let _ = io::stdin().read_line(&mut fibo_index);
    let selected_index = fibo_index.trim().parse::<i32>().unwrap();
    if selected_index > 0 { mod_print(format!("Your selected index in the Fibonacci sequence is: [{}]\n", selected_index).as_str()); }
    selected_index
}

fn provide_result(index: i32) -> () {
    let (suffix, result) = get_fibo_vars(index);
    mod_print(format!("The {}{} number of the Fibonacci sequence is {}.", index, suffix, result).as_str());
}

fn get_fibo_vars(index: i32) -> (String, u64) {
    (get_suffix(index), get_nth_fibonacci_number(index))
}

fn get_suffix(index: i32) -> String {
    let suffix = match index {
        1 => "st".to_string(),
        2 => "nd".to_string(),
        3 => "rd".to_string(),
        _ => "th".to_string(),
    };
    suffix
}

fn get_nth_fibonacci_number(index: i32) -> u64 {
    let mut first: u64 = 1;
    let mut second: u64 = 1;
    
    let result = match index {
        1 => first,
        2 => second,
        _ => {
            let mut next: u64 = 0;
            for _ in 3..=index {
                next = first + second;
                first = second;
                second = next;
            }
            next
        }
    };
    result
}

fn string_joiner(number_array: [i32; 11]) -> String {
    number_array.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join("\t")
}







