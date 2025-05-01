mod main2;

fn main(){
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
    let numbers: [i32; 5] = [1,2,3,4,5];
    let fruits: [&str; 3] = ["apple", "pear", "lie"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array 1st Element: {}", fruits[0]);
    println!("Fruits Array 2st Element: {}", fruits[1]);
    println!("Fruits Array 3st Element: {}", fruits[2]);
    
    // Tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);
    let my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);
    
    //Slices: [1,2,3,4,5]
    
}