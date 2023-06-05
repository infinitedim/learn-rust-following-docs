// This is Imutable variable, uncomment to run 
// fn main() {
//     let x = 5;
//     println!("the value of x is: {x}");
//     x = 6;
//     println!("the value of x is: {x}");
// }


// This is Mutable variable, uncomment to run 
// fn main() {
//     let x = 5;
//     println!("the value of x is: {x}");
//     x = 6;
//     println!("the value of x is: {x}");
// }

// This is scope variable and shadowing variable in Rust, uncomment to run
// fn main() {
//     let x = 5;
    
//     let x = x + 1; 
    
//     {
//         let x = x * 2;

//         println!("The value of x at inner scope is: {x}")
//     }

//     println!("the value of x is: {x}");
// }

// Shadowing example 2
// fn main() {
//     // shadowing is allow you to reasign, different with mutable, 
//     // if you're change variable to mutable, compiler will give you an error
//     let spaces = "   ";
//     let spaces = spaces.len();
// }

// This is constants in Rust, uncomment to run
fn main() {
    const THREE_HOURS_IN_SECOND: u32 = 60 * 60 * 3; 

    println!("The value of Three hours in second is: {THREE_HOURS_IN_SECOND}");
}
