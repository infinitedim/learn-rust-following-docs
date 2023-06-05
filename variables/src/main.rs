// This is Imutable variable, run this will be error, make sure you comment if you want to run other function without error
// fn imutable() {
//     let x = 5;
//     println!("the value of x is: {x}");
//     x = 6;
//     println!("the value of x is: {x}");
// }


// This is Mutable variable 
fn mutable() {
    let mut x: i32 = 5;
    println!("the value of x is: {x}");
    x = 6;
    println!("the value of x is: {x}");
}

// This is scope variable and shadowing variable in Rust
fn scope() {
    let x: i32 = 5;
    
    let x: i32 = x + 1; 
    
    {
        let x: i32 = x * 2;

        println!("The value of x at inner scope is: {x}")
    }

    println!("the value of x is: {x}");
}

// Shadowing example 2
fn shadowing() {
    // shadowing is allow you to reasign, different with mutable, 
    // if you're change variable to mutable, compiler will give you an error
    let spaces: &str = "   ";
    let spaces: usize = spaces.len();

    println!("{spaces}")
}

// This is constants in Rust
fn constants() {
    const THREE_HOURS_IN_SECOND: u32 = 60 * 60 * 3; 

    println!("The value of Three hours in second is: {THREE_HOURS_IN_SECOND}");
}

fn main() {
    // imutable() // uncomment this line to run imutable function
    mutable();
    scope();
    shadowing();
    constants();
    println!("just make sure main function available")
}