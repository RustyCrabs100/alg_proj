use bigdecimal::{BigDecimal, One};
use num_rational::BigRational;
use std::str::FromStr;

fn pow_new(mut base: BigDecimal, mut exp: u32, prec: u64, extra: u64) -> BigDecimal {
    let mut acc = BigDecimal::one();
    let working_prec = prec + extra;

    while exp > 0 {
        if exp & 1 == 1 {
            acc = (acc * &base).with_prec(working_prec);
        }
        exp >>= 1;
        if exp > 0 {
            base = (&base * &base).with_prec(working_prec);
        }
    }
    acc.with_prec(prec)

    // To simplify: result = val1 ^ val2
    // pow_new returns result when called with the appropriate inputs.
}
// SA = 2(PI)r^2 + 2(PI)(r)(h)

fn main() {
    let prec: u64 = 10000;
    let extra: u64 = 100;
    let big_decimal_3: BigDecimal = BigDecimal::from(3u8);
    let big_decimal_2: BigDecimal = BigDecimal::from(2u8);
    let big_decimal_9: BigDecimal = BigDecimal::from(9u8);
    let big_decimal_200: BigDecimal = BigDecimal::from(200u16);

    // 2000 digits of Pi
    // I could make this larger, but with the issues of Windows Powershell & Due to using massive numbers making processing extremely slow, I decided not to.
    // Besides, I'm already witnessing floating point errors, even though this code is using arbitrary precision to calculate floating pointers.
    let pi_str = include_str!("10000_digits_of_pi.txt");
    let big_decimal_PI: BigDecimal = BigDecimal::from_str(pi_str)
        .unwrap()
        .with_prec(prec + extra)
        .normalized();
    println!("Big PI: {}", &big_decimal_PI.to_string());
    // h = 200 (This was the chosen value for the can) divided by 9 times PI (with 2000 digits)
    let big_decimal_height: BigDecimal =
        BigDecimal::from(&big_decimal_200 / (&big_decimal_9 * &big_decimal_PI))
            .with_prec(prec + extra)
            .normalized();
    println!("Big Height: {}", big_decimal_height.to_string());
    // 2(PI)r^2
    // Using my own custom implementation because for some reason powf and powi are not imported...
    let first_value: BigDecimal = BigDecimal::from(
        &big_decimal_2 * &big_decimal_PI * pow_new(big_decimal_3.clone(), 2, prec, extra),
    )
        .with_prec(prec + extra)
        .normalized();
    println!("First Big Value: {}", first_value.to_string());
    // 2(PI)(r)(h)
    // Turns into 400 / 3 is that's what big_decimal_2 * big_decimal_PI * big_decimal_3 * big_decimal_height
    // First turn it into a BigRational, then convert to a BigDecimal
    let second_rational_val: BigRational = BigRational::new(400.into(), 3.into());
    let second_value: BigDecimal = {
        let numer= BigDecimal::from(second_rational_val.numer().clone());
        let denom = BigDecimal::from(second_rational_val.denom().clone());
        (numer / denom).with_prec(prec + extra)
    }.normalized();
    println!("Second Big Value: {}", second_value.to_string());
    // final_value = SA = 2(PI)r^2 + 2(PI)(r)(h)
    let final_value: BigDecimal = BigDecimal::from(first_value + second_value)
        .with_prec(prec);
    println!(
        "Final Big Value (Second Big Value + First Big Value): {}",
        final_value.to_string()
    );
}