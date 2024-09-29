// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    let result = num * num;

    return result;
}

fn power_of_n(num: i32, num2: i32) -> i32 {
    let mut result = 1;
    for _i in 0..num2 {
        result *= num;
    }

    println!("{num:} raise to the power of {num2:} is equals to {result:}.");

    return result;
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");

    let base: i32 = 3;
    let exponent: i32 = 3;

    power_of_n(base, exponent);
}
