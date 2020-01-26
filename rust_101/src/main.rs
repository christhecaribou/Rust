#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rand::seq::SliceRandom; // 0.7.2

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
    println!("A number: {}", sum+difference+product
        +quotient+remainder);

    // A 'constant' should be formatted C / Rust style
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS: {}", MAX_POINTS);

    // Rust’s char type is four bytes in size and 
    // represents a Unicode Scalar Value
    // This means you can use Emoji among other things
    let mut lamb = String::from("αβγδ");
    let veal = "εζηθ";   
    let cat = '😻'; // This is a char, not a string
    
    // I had to read a thread on the Rust forums debating
    // the best way to concatenate strings AND chars
    // a fundamental operation  2019-01-01
    let pork = format!("{}{}{}", lamb, veal, cat);
    
    // This is the correct way to do it,
    // as it's more effecient with memory
    lamb.push_str(veal);
    lamb.push(cat);
    println!("{}", pork);
    println!("{}", lamb);

    // A compound data type, a TUPLE
    // optional type annotations make inference easier
    let tup0: (i32, f64, u8) = (500, 6.4, 1);
    let tup1 = (420, "abcd", 69);
    
    // The :? tells Rust to allocate the maximum
    // amount of memory, otherwise, the command
    // can't determine data structure size
    println!("{:?}{:?}", tup0, tup1);

    // Destructure
    let tup = (500, 6.4, 1);
    
    // Unused variables should have underscore prefix
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    // Helpful accessors
    let q: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = q.0;
    let _six_point_four = q.1;
    let _one = q.2;

    // Arrays use stacks, not heaps
    let _array = [1, 2, 3, 4, 5];
    
    // Vectors
    let months = ["January", "February", "March",
                  "April", "May", "June",
                  "July", "August", "September",
                  "October", "November", "December"];
     let sample: Vec<_> = months
        .choose_multiple(&mut rand::thread_rng(), 1)
        .collect();

    // Random month
    println!("{:?}", sample); 

    // if condition is expression, can use with let
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    // Ignite the rocket, a simple webserver 
    //rocket::ignite().mount("/", routes![index]).launch();
}
