fn main() {
    let imutable_variable = 5;
    let mut mutable_variable = 5;
    println!("The value of imutable_variable is: {imutable_variable}");
    println!("The value of mutable_variable is: {mutable_variable}");
    mutable_variable = 6;
    println!("The value of mutable_variable is: {mutable_variable}");
    
    let a = 0.5;
    {
        let a = 10;
        println!("The value of a is: {a}");
    }

    println!("Hello, world: {a}");
}
