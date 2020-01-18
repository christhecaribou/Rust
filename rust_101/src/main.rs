#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    println!("Hello, world!");

    // Rustaceans shadow their variables
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
    
    // A 'constant' should be formatted C & Rust share style
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS: {}", MAX_POINTS);

    // Ignite the rocket
    //rocket::ignite().mount("/", routes![index]).launch();
}
