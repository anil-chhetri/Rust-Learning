fn main() {
    println!("Hello, world!");

    // deconstructing 
    let _y: i8 = 2;
    let(a, b)= (2 as i8, 'b');

    println!("a {}, b {}", a, b.to_string());

    // match operator

    let number = 3_i8;

    match number {
        1 => println!("1st matched:  value is {}", number),
        2 | 3 => println!("either 2 or 3"),
        _ => println!("something else")
    };



    // match operator helping to unpack values. 

    enum Message {
        Quit,
        Move {x: i32, y: i32}
    }

    let message = Message::Move { x: 2, y: 3 };

    match message {
        Message::Quit=> println!("Quit"),
        Message::Move { x, y } => println!("Move to {} {}", x, y)
    };

    let quite_message = Message::Quit;

    match quite_message {
        Message::Quit=> println!("Quit"),
        Message::Move { x, y } => println!("Move to {} {}", x, y)
    };

    // if let operator similar to match: checks 1 sepcific pattern and ignore others.
    let favorite_color: Option<&str> = Some("blue");

    if let Some("blue") = favorite_color {
        println!("your favriote color is blue.");
    }

    //  unpacking value of with if let.
    let some_value: Option<i8> = Some(43);

    if let Some(x) = some_value {
        println!("unpacked value is {}", x);
    }

    



}
