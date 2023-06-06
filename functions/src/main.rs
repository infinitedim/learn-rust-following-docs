
fn main() {
    println!("Hello, world!");

    another_function();
    function_with_params(2);
    print_labeled_measurement(5, 'h');
    // statement_found_expressin_fn();
    expression_part_of_statement();
    let returns: i32 = function_with_return();

    println!("The value of returns is: {returns}");

    let six = plus_one(5);

    println!("The value of plus_one is: {six}");

    // let e = plus_one_error(5);
    // println!("The value of plus_one_error is: {plus_one_error}")
}

fn another_function() {
    println!("Another function");
} 

fn function_with_params(params: i32) {
    println!("the value of params is: {}", params);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// this function will be error because compiler found a statement but you write an expression
// you should remove the parentheses in x variable
// fn statement_found_expression_fn() {
//     let x = (let y = 6;)
// }

fn expression_part_of_statement() {
    let y: i32 = {
        // This scope is expression
        let x: i32 = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn function_with_return() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
// Uncomment to see the error
// error[E0308]: mismatched types
// fn plus_one_error(x: i32) -> i32 {
//     x + 1;
// }
