mod main2;

fn main() {
    //Primitive Data Types
    println!("====================Primitive Data Types=================================");
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    let pi: f64 = 3.141592653589793;
    println!("Value of pi: {}", pi);
    //Boolean Values
    let is_snowing: bool = true;
    println!("Value of is_snowing: {}", is_snowing);
    //Charachter Type - char
    let letter: char = 'a';
    println!("First letter of the alphabet: {}", letter);

    // Compound Data Types
    println!("====================Compound Data Types=================================");
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let fruits: [&str; 3] = ["apple", "pear", "lie"];
    let airplains: [&str; 3] = ["Aribaus A320", "Boeing 737 Max", "Aribus A380"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array 1st Element: {}", fruits[0]);
    println!("Fruits Array 2st Element: {}", fruits[1]);
    println!("Fruits Array 3st Element: {}", fruits[2]);
    //Print airplains
    println!("AirPlains: {:?}", airplains);

    // Tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);
    let my_mix_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);

    //Slices: [1,2,3,4,5]
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number slices: {:?}", number_slices);
    
    let animals_slices :&[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animals Slices: {:?}", animals_slices);

    let book_slices :&[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"ZEN".to_string()];
    println!("Book Slices: {:?}", book_slices);
    
    //String vs String Slices(&str)
    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone Cold Says: {}", stone_cold);
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);
    
    //B- &str (String Slice
    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);
}
