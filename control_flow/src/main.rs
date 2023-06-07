fn main() {
    println!("\n");
    println!("======IF======");
    main_if();
    println!("====END IF====\n");
    println!("======LOOP======");
    main_loop();
    println!("====END LOOP====\n");
    println!("======WHILE======");
    main_while();
    println!("====END WHILE====\n");
    println!("======FOR======");
    main_for();
    println!("====END FOR====\n");
}

fn main_if() {
    if_expression();
    // wrong_if(); // Uncomment to run this code
    right_if();
    if_multiple_condition();
    if_in_variable();
}

fn if_expression() {
    let num: i32 = 3;

    if num < 5 {
        println!("The condition was true");
    } else {
        println!("The condition was false");
    }
}

// Uncomment to run this code
// error[E0308]: mismatched types
// fn wrong_if() {
//   let num = 5;

//   if num {
//     println!("The conditin was true");
//   }
// }

fn right_if() {
    let number: i32 = 3;

    if number != 0 {
        println!("Number was something other than zero")
    }
}

fn if_multiple_condition() {
    let number: i32 = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_in_variable() {
    let condition: bool = true;
    let number: i32 = if condition { 5 } else { 6 }; // right
                                                     // let number2 = if condition { 5 } else { "six" } //wrong;

    println!("The value of number is: {number}");
}

// Loop

fn main_loop() {
    // looping_with_loop();
    return_value_from_loop();
    nested_loop();
}

// This code wil be infinite loop, uncomment to try it
// fn looping_with_loop() {
//     loop {
//         println!("again");
//     }
// }

fn return_value_from_loop() {
    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");
}

fn nested_loop() {
    let mut count: i32 = 0;
    'continuing_up: loop {
        println!("count: {count}");
        let mut remaining: i32 = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'continuing_up;
            }
            remaining -= 1
        }
        count += 1
    }

    println!("End count: {count}")
}

fn main_while() {
    looping_with_while();
    while_example();
}

fn looping_with_while() {
    let mut number: i32 = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn while_example() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn main_for() {
    looping_with_for();
    for_example();
}

fn looping_with_for() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn for_example() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}