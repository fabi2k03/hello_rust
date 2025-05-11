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

    //Reverse Numbers
    println!("\nReverse Numbers:");
    reverse_range();

    println!("\n====================Ownership=================================\n");
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let x = 5;
    let y = x;

    let mut s1 = String::from("hello");
    s1 = String::from("ahoi");

    println!("{}, world!", s1);

    let mut s2 = String::from("hello");
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);

    let s4 = String::from("hello");
    take_ownership(s4);                         // s's value moves into the function...
                                                // ... and so is no longer valid here
    // println!("{s4}!");
    let x1 = 5;
    make_copy(x1);                              // because i32 implements the Copy trait,
                                                // x does NOT move into the function,
                                                    // so it's okay to use x afterward
    println!("x1 = {}", x1);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    let s1 = String::from("hello");
   // let (s2, len) = calculate_length(s1);

   // println!("The length of {} is {}", s2, len);

    println!("\n====================Reference and Borrowing=================================\n");
    //Reference and Borrowing
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The len length of {s1} is {len}.");

    //Mutable References
    let mut s = String::from("hello");
    change(&mut s);
    
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    }// r1 goes out of scope here, so we can make a new reference with no problems.as 
    let r2 = &mut s;

    println!("\n====================The Slice Type=================================\n");
    let mut sSlice = String::from("hello world");
    let word = first_word(&sSlice);
    
    //String slices
    let hello = &sSlice[0..5];
    let world = &sSlice[6..11];
    
    let len = sSlice.len();
    let slice = &sSlice[3..len];
    let slice = &sSlice[3..];
    
    println!("The slice type is: **{}** and **{}**", hello, world);
    
    let my_string = String::from("hello world");
    //`first_word` works on slices of `string`, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);
    //added comment
    
    let my_string_literal = "hello world";
    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
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

fn reverse_range(){
    for number in (1..4+1).rev(){
        println!("{}", number);
    }
}

fn take_ownership(some_string: String){
    println!("{some_string}");
}// Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn make_copy(some_integer: i32) {
    println!("{some_integer}");
}// Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String{
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String{
    a_string
}

fn calculate_length(s: &String) -> usize{
    s.len()
}

fn change(some_string: &mut String){
    some_string.push_str(", world");
}

fn first_word(s: &str)-> &str{
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    
    &s[..]
}

