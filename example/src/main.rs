use std::{fs::File, io};

fn process_data(data: &[i32]) {
    let mut sum = 0;

    for item in data {
        sum += item;
    }

    println!("Sum of data: {}", sum);

    if sum % 2 == 0 {
        println!("The sum is even.");
    } else {
        println!("The sum is odd.");
    }
}

fn split_string(s: String, delimeter: char, fileds: usize) -> String {
    let part: Vec<&str> = s.split(delimeter).collect();
    let result = part.get(fileds);
    result.expect("something went wrong!").to_string()
}

fn own_int(int_object: i32) {
    println!("{}", int_object)
}

fn own_string(s: &String) {
    println!("string is owned {}", s);
}

fn own_vector(v: &Vec<i32>) -> Vec<i32> {
    let mut new_vector_object = v.to_vec();
    new_vector_object.push(10);
    new_vector_object
}

fn borrowing_example() -> () {
    let vec_array = vec![1, 2, 3, 4, 5, 6];
    let int_object = 10;
    let string_object = "hello world!".to_string();

    own_int(int_object);
    println!(
        "int object after owning by different function {}",
        int_object
    );

    own_string(&string_object);

    println!(
        "string object after owning by different function: and only works after borrowing the value with & {}",
        string_object
    );

    own_vector(&vec_array);

    println!(
        "similar case as string, value is not accessiable here, so to make it assesiable just borrow vector with &"
    );
    println!("{:?}", vec_array);
}

// fn loop_and_panic(v: Vec<i32>) {
//     for n in v {
//         if n < 0 {
//             panic!("negative number found");
//         }

//         println!("found positive number: {}", n);
//     }
// };

fn not_panicking_example() {
    let file = File::open("invalid_file.txt");
    match file {
        Ok(_) => {
            println!("everything is okay.")
        }
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
                println!("file not found");
            }
            _ => {
                println!("cant open the file.");
            }
        },
    };
}

fn vector_operation() {
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let mut another_vector = Vec::new();

    another_vector.push(10);
    another_vector.push(20);
    another_vector.push(30);

    println!("presenting new vector {:?}", vector);
    println!("presenting another vector {:?}", another_vector);

    println!("removing entity from both vector");

    vector.pop();
    another_vector.pop();

    println!("presenting new vector {:?}", vector);
    println!("presenting another vector {:?}", another_vector);

    println!("extending the vector");

    vector.append(&mut another_vector);
    println!("new vector: {:?}", vector);
    println!("appended vector: {:?}", another_vector);

    let mut another_vector_2 = vec![2, 3, 4, 5, 6, 7, 8];
    vector.append(&mut another_vector_2);
    println!("next new vector {:?}", another_vector_2);
    println!("old vector {:?}", vector);
}

fn main() {
    process_data(&[1, 2, 3, 4, 5]);
    println!("Data processed successfully.");
    let result = split_string("hello, world!".to_string(), ',', 0);
    println!("{}", result);
    borrowing_example();

    // let new_vector_object = vec![1, 2, 3, 45, -5];
    // loop_and_panic(new_vector_object);

    not_panicking_example();

    println!("=====================");

    vector_operation();
}
