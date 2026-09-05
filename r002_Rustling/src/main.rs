//const PI: f64 = 3.1415; //We can assign and allocate Constants in Global Scope

//let PII: f64 = 3.1415;  // We cannot use let for global variables

fn main() {
    let mut number = 10;
    println!("number = {number}");

    number += 1;
    println!("number = {number}");

    //Naming convention for Variables -> Snake Case for Rust
    let first_name = "Federico";
    println!("My name is: {first_name}");

    //Naming convention for Constants -> All Uppercase with Snake Case
    const ONE_MINUTE: i32 = 60;
    const ONE_HOUR: i32 = ONE_MINUTE * 60;

    println!("One minute is: {ONE_MINUTE}s");
    println!("One hour is: {ONE_HOUR}s");

    const PI: f64 = 3.1415;
    println!("PI is: {PI}");

}
