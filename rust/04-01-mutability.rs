fn main() {
    let _immutable_variable = 1i;
    let mut mutable_variable = 1i;

    println!("Before mutation: {}", mutable_variable);

    mutable_variable += 1;

      println!("After mutation: {}", mutable_variable);

      //  _immutable_variable += 1;
}
