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
    //let numbers: [i32; 5] = [1, 2, 3, 4, 5];
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
    println!("====================Functions=================================");
    another_function();
    let x: i32 = 5;
    let y: i32 = 10;
    print!("Addtion: {} + {}",x, y);
    let r: i32 = addition(x,y);
    println!(" = {}", r);
    print_labeled_measurement(33, 'h');
    println!("====================Control Flow=================================");
    println!("IF-STATEMENTS:");
    let mut number: i32 = 3;

    if number < 5 {
        println!("condition (Number<5) was true");
    }else{
        println!("condition (Number<5>) was false");
    }

    number = 7;

    if number < 5 {
        println!("condition (Number<5) was true");
    }else{
        println!("condition (Number<5>) was false");
    }

    let number2: i32 = 6;

    if number2 % 4 == 0{
        println!("number2 = {} is divisible by 4", number2);
    } else if number2 % 3 == 0{
        println!("number2 = {} is divisible by 3", number2);
    }else if number2 % 2 == 0{
        println!("number2 = {} is divisible by 2", number2);
    }else{
        println!("number2 = {} is not divisible by 4, 3 and 2", number2);
    }

    let condition: bool = true;
    let number3 = if condition{ 5 }else{6};
    println!("The value of number3: {}", number3);

    //Loops
    println!("LOOPS:");
    let mut counter = 0;
    let result = loop{
        counter += 1;

        if counter == 10{
            break counter *2;
        }
    };

    println!("Result: {}", result);

    println!("Multiple Loops:");
    println!("---------------------");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9{
                break;
            }
            if count == 2{
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count {}", count);

    println!("\nWhile-Loop:");
    number = 3;

    while number != 0 {
        println!("number = {}", number);
        number -= 1;
    }

    println!("\nfor-Loop:");
    let a:[i32; 5] = [10,20,30,40,50];

    for i in 0..a.len(){
        println!("Element at index {}: {}", i, a[i]);
    }
}

fn another_function(){
    println!("Another function.");
}

fn addition(x: i32, y: i32) -> i32 {
    let result = x + y;
    return result;
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
