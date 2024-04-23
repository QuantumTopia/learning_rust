use crate::utils::print_utils::mod_print as mod_print;

pub fn example_stack() 
{
    // iterating_over_string();
    get_first_word_using_slice();
}

fn iterating_over_string()
{
    mod_print("Iterating over string byte by byte:");
    let s = String::from("Hello world!");

    // split string into chars
    let bytes = s.as_bytes();

    // create iterator over collection
    // enumerate returns a tuple from the iterator, first item is the index, second is ref to item
    let bytes_iter = bytes.iter().enumerate();

    // loop over iterator with for loop
    for (index, &item_ref) in bytes_iter
    {
        mod_print(format!("{}", item_ref as char).as_str());
    }
}

fn get_first_word_using_slice() 
{
    mod_print("Get first word from string using slice:");

    let s = String::from("Hello world!");
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            mod_print(format!("{:?}", &s[0..index]).as_str());
            return;
        }
    }

    mod_print(format!("{:?}", &s[..]).as_str());
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn using_an_enum() 
{
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}