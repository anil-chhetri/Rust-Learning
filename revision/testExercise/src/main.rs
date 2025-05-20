fn interproduct(a: i32, b: i32, c:i32) -> i32 {
    a*b+b*c+c*a

    // (a*b).saturating_add(b*c).saturating_add(c*a)
}

fn fib(n:u32) -> u32 {
    if n < 2 {
        return n;
    }
    else {
        return fib(n-1) + fib(n-2);
    }
}


fn main() {
    println!("Hello, 🌍!");

    let mut x: i32 = 10;
    println!("x: {x}");

    x = 20;
    println!("X: {x}");


    let result:i32 = interproduct(120, 100, 248); 
    // 32767
    println!("result: {result}");

    let n = 3;
    println!("fib {n}: {}", fib(n));
}
