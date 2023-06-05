// see https://doc.rust-lang.org/book/ch03-02-data-types.html for details
use std::io;

// Integer data types
fn int() {
    let x: i32 = 2;

    let y: i32 = 3;
    println!("======INTEGER======");
    println!("x is: {x} \n y is: {y}");
}

// Floating data types
fn float() {
    let x: f64 = 2.0; // f64

    let y: f32 = 3.0; // f32

    println!("======FLOAT======");
    println!("x is: {x} \n y is: {y}");
}

// Numeric Operation
fn numeric_ops() {
    println!("======NUMERIC OPERATION======");
    // addition
    let sum: i32 = 5 + 10;
    println!("5 + 10 = {sum}");

    // subtraction
    let difference: f64 = 95.5 - 4.3;
    println!("95.5 + 4.3 = {difference}");

    // multiplication
    let product: i32 = 4 * 30;
    println!("4 * 30 = {product}");

    // division
    let quotient: f64 = 56.7 / 32.2;
    let truncated: i32 = -5 / 3; // Results in -1
    println!("56.7 / 32.2 = {quotient}");
    println!("-5 / 3 = {truncated}");

    // remainder
    let remainder: i32 = 43 % 5;
    println!("43 % 5 = {remainder}");
  }

// Character data types
fn char() {
  println!("======CHARACTER======");
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat: char = '😻';

    println!("{c} \n {z} \n {heart_eyed_cat}");
}

// Boolean
fn bool() {
  println!("======BOOLEAN======");
    let t = true;
    let f: bool = false; // with explicit type annotation

    println!("{t} \n {f}")
  }
  
  // Tuple data types
// fn tuple() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);

//     println!("{tup}");
// }

// Destructing tupple value
fn tuple_destruct() {
  println!("======TUPLE======");
  let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
  }

// Access directly tuple value with period ( . )
fn tuple_period() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("{five_hundred}");
    println!("{six_point_four}");
    println!("{one}");

}

fn array() {
    //   let months = [
    //     "January",
    //     "February",
    //     "March",
    //     "April",
    //     "May",
    //     "June",
    //     "July",
    //     "August",
    //     "September",
    //     "October",
    //     "November",
    //     "December",
    // ];

    // explicitly write type for each element in array
    // let a: [i32; 5] = [1, 2, 3, 4, 5]; // this mean type i32 for five data

    // initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon
    // let b = [3; 5]; // same as let b = [3, 3, 3, 3, 3]

    println!("======ARRAY======");
    let c = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = c[index];

    println!("The value of the element at index {index} is: {element}");
}

fn main() {
    int();
    float();
    numeric_ops();
    char();
    bool();
    // tuple();
    tuple_destruct();
    tuple_period();
    array();
}
