fn main() {
    let x: u32 = 5;
    let y: f32 = 3.0; // float 32bit
    let f: bool = false; //boolean
    let z: char = 'ℤ'; //char
    let tup: (i32, f64, u8) = (500, 6.4, 1); //tup
    let (x, y, z) = tup; //like destructuring in js
    let a = [1, 2, 3, 4, 5]; // same as tup but with same type
    let stringArray = ["dsa", "asd"]; // array of string
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    let returnNumber = with_return();
    println!("Return: {returnNumber}");
}
fn with_return() -> i32 {
    1 + 1
}
fn array() {
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // here i32 is type and :5 is length
    let b = [3; 5]; //inside[] ; means length so it's similar to [3,3,3,3,3]

    let first = a[0];
}
fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1); //tup
    let (x, y, z) = tup; //like destructuring in js

    let firstNumber = tup.0;
    let secondNumber = tup.1;
    let thirdNumber = tup.2;
}
fn numeric_operation() {
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
