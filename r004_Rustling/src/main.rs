fn main() {
    let user_input = "100";
    let converted: u32 = user_input.parse().expect("Could not parse...");

    println!("Converted= {}", converted);
    //Scaler data types
    let number:i8 = 10;
    let pi:f32 = 3.1415;
    let turned_on:bool = false;
    let delta:char = 'D';

    //Compound data types
    let coordinates: (f32, f32) = (1.5, 2.5); //Tuple
    let people: [&str; 3] = ["Bob", "Luigi", "Ashley"]; //Array of strings
}
