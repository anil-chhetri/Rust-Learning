
fn main() {
    
    println!("different max value of i32 and u32");
    let max_value_i32: i32 = i32::max_value();
    let max_value_u32: u32 = u32::max_value();

    println!("max value of i32 is {}", max_value_i32);
    println!("max value of u32 is {}", max_value_u32);


    println!("difference between String and &str");

    // String - owned, heap-allocated
    // We can modify it
    let mut owned_string: String = String::from("Hello ");
    owned_string.push_str("world");
    owned_string += "!";

    println!("{}", owned_string);

    assert_eq!(owned_string, "Hello world!");

    // immutable version of string can't add value to it.
    // let test_string: String = "This is test message".to_string();
    // test_string.push_str("adding more text to it.");


    /*
    Good question! The `&` symbol in `&owned_string` is being used to create a reference to the `owned_string` value. Let me explain the reasoning behind this:

        When we use `&owned_string[0..5]`, we're:

        1. First creating a reference (`&`) to `owned_string`
        2. Then using the slice syntax `[0..5]` to create a slice of the first 5 characters

        This is necessary because:

        - The slice operation (`[..]`) in Rust works on references, not on owned values directly
        - We want to create a view into the existing data without taking ownership of it
        - Using `&` creates a borrowed reference instead of moving or copying the data

        If we tried to use `owned_string[0..5]` without the `&`, we would get a compiler error because Rust expects to use the slice operator on a reference.

        This pattern is fundamental to Rust's borrowing system - we're "borrowing" access to part of the string data without taking ownership. The `&str` type is specifically designed to be a borrowed view into string data, whether that data comes from a `String` or a string literal.
    
     */

    let string_slice: &str = &owned_string[2..5];
    println!("sliced string: {}", string_slice);


    let _array_var: [u8; 2] = [1, 2]; 

    let _initialize_array:[i8; 10] =  [0; 10];

    println!("{}", _initialize_array[9]);


    let fruits: [String; 5] = ["apple".to_string(), "banana".to_string(), "cherry".to_string(), "date".to_string(), "elderberry".to_string()];

    // Using index
    let _first = &fruits[0]; // "apple"

    println!("the first fruit is {}", _first);

    // Using get method (returns Option<&T>)
    match fruits.get(1) {
        Some(fruit) => println!("The second fruit is {}", fruit),
        None => println!("No fruit at this index"),
    }


    match fruits.get(20) {
        Some(fruit) => println!("The second fruit is {}", fruit),
        None => println!("No fruit at this index 12"),
    }


    println!("all values of fruits {:?}", fruits);

    let _first_name: String = String::from("anil");
    let _second_name: String = String::from("anilc");

    // same lifeline
    let result = longest_name("anil", "garima");

    println!("{:?}", result);

    // difference lifeline
    let first_name: String = String::from("anil");
    let result2; 
    let second_name: String = String::from("anilc");
    {
        // if using this second name we get error.
        // let second_name: String = String::from("anilc");  
        result2 = longest_name(&first_name, &second_name);
    }

    println!("{:?}", result2);


}



fn longest_name<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() { first } else { second }
}
