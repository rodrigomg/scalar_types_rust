fn main() {
    floating_point();
    numeric_operations();
    booleans();
}

fn floating_point() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("Varible x: {}", x);
    println!("Varible y: {}", y);
}

fn numeric_operations() {
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;

    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("product: {}", product);
    println!("quotient: {}", quotient);
    println!("remainder: {}", remainder);
}

fn booleans() {
    let t = true;
    let f: bool = false; // with explicit type annotation

    println!("variable t: {}", t);
    println!("variable f: {}", f);
}

