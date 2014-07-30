use std::num;

fn main() {
    let ret = sqroot(36.0, 0.3);
    print!("The square root is: {}\n", ret);
}

fn sqroot(number: f64, error: f64) -> f64 {
    let mut previous_estimate;
    let mut current_estimate;

    // Ensure the value we're working with is positive
    if number <= 0.0 {
        print!("The number must be greater than 0 (found {})", number);
        return 0.0;
    }

    // Ensure the error bound is positive
    if error <= 0.0 {
        print!("The error bound must be positive (found {})", error);
        return 0.0;
    }

    current_estimate = number / 2.0;

    loop {
        previous_estimate = current_estimate;
        current_estimate = (previous_estimate + number/previous_estimate) / 2.0;

        // Is the difference small enough yet?
        if std::num::abs(current_estimate - previous_estimate) < error {
            break;
        }
    }

    return current_estimate;
}
