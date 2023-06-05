use std::io;

// array
fn main() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // explicitly write type for each element in array
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // this mean type i32 for five data

    // initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon
    let b = [3; 5]; // same as let b = [3, 3, 3, 3, 3]

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
