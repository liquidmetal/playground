fn main() {
    let a_variable;
    {
        let x = 2i;
        a_variable = x * x;
    }

    println!("a_variable = {}", a_variable);

    let another_variable;

    //println!("another_variable = {}", another_variable);

    another_variable = 1i;
    println!("another variable = {}", another_variable);
}
