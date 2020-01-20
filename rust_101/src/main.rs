#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    println!("Hello, world!");

    // Rustaceans "shadow" their variables
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
    
    // Basic operations
    let sum = 5 + 10;
    let difference = 95 - 4;
    let product = 4 * 30;
    let quotient = 2 / 1;
    let remainder = 43 % 5;
    
    // You must USE data structures or Rust will complain
    println!("A number: {}", sum+difference
        +product+quotient+remainder);

    // A 'constant' should be formatted C / Rust style
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS: {}", MAX_POINTS);

    // Rust’s char type is four bytes in size and 
    // represents a Unicode Scalar Value
    // This means you can use Emoji among other things
    let mut a = String::from("αβγδ");
    let b = "εζηθ";
    
    // This is a char, not a string
    let c = '😻';
 
    // I had to read a thread on the Rust forums debating
    // the best way to concatenate strings AND chars
    // a fundamental operation  2019-01-01
    let pork = format!("{}{}{}", a, b, c);
    
    // This is the correct way to do it,
    // as it's more effecient with memory
    a.push_str(b);
    a.push(c);
    a.push_str("d");
    println!("{}", pork);
    println!("{}", a);

    // Ignite the rocket
    //rocket::ignite().mount("/", routes![index]).launch();
}
