fn main() {
    //Scope exmaple
    let n = 5;

    {
        //let n = 10;
        println!("inner n is: {n}");
    }

    println!("outer n is: {n}");

    let spaces = "      ";
    //let length_of_spaces = spaces.len();
    let spaces = spaces.len();

    println!("Spaces: {spaces}");
}
