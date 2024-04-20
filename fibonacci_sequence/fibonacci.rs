use std::io;

fn main() {
    welcome_message();
    let mut selected_index = process_input();
    provide_result(selected_index);
    loop {
        println!("Would you like to know another number from the sequence?");
        selected_index = process_input();
        provide_result(selected_index);
    }
    
}

fn welcome_message() -> () {
    println!("Welcome to the Fibonacci number machine!");
    println!("The beginning sequence looks like:");
    let numbers = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
    let indexes = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    print
    println!("Which Fibonacci number would you like to know?");
}

fn process_input() -> i32 {
    let mut fibo_index = String::new();
    let _ = io::stdin().read_line(&mut fibo_index);
    let selected_index = fibo_index.trim().parse::<i32>().unwrap();
    println!("Your selected index in the Fibonacci sequence is: [{}]", selected_index);
    selected_index
}

fn provide_result(index: i32) -> () {
    let (suffix, result) = get_fibo_vars(index);
    println!("The {}{} number of the Fibonacci sequence is {}.", index, suffix, result);
}

fn get_fibo_vars(index: i32) -> (String, i32) {
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

fn get_nth_fibonacci_number(index: i32) -> i32 {
    let mut first: i32 = 1;
    let mut second: i32 = 1;
    
    let result = match index {
        1 => first,
        2 => second,
        _ => {
            let mut next: i32 = -1;
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

fn string_joiner(number_array: [i32]) -> String {
    number_array.into_iter().map(|i| i.to_string()).collect::<String>().join("\t")
}







