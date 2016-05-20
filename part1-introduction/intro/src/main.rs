fn main() {
    // NUMBERS

    // Rust can use numbers without needing the type specified
    let a = 10;
    let b = 1;
    let c = a + b;
    println!("c is {}", c);

    // But if you want to specify a type in Rust this is the syntax
    let d: i8 = 100;
    let e: f32 = 0.42;
    // and so on...

    // This value is too high for an 8 bit integer, so the compiler will warn you
    let f: i8 = 200;

    // Let's print the min and max value for each type so you get a feel of when to use them
    println!("i8 MIN {}", std::i8::MIN);
    println!("i8 MAX {}", std::i8::MAX);
    println!("i16 MIN {}", std::i16::MIN);
    println!("i16 MAX {}", std::i16::MAX);
    println!("i32 MIN {}", std::i32::MIN);
    println!("i32 MAX {}", std::i32::MAX);
    println!("i64 MIN {}", std::i64::MIN);
    println!("i64 MAX {}", std::i64::MAX);
    println!("u8 MIN {}", std::u8::MIN);
    println!("u8 MAX {}", std::u8::MAX);
    println!("u16 MIN {}", std::u16::MIN);
    println!("u16 MAX {}", std::u16::MAX);
    println!("u32 MIN {}", std::u32::MIN);
    println!("u32 MAX {}", std::u32::MAX);
    println!("u64 MIN {}", std::u64::MIN);
    println!("u64 MAX {}", std::u64::MAX);
    println!("f32 MIN {}", std::f32::MIN);
    println!("f32 MAX {}", std::f32::MAX);
    println!("f64 MIN {}", std::f64::MIN);
    println!("f64 MAX {}", std::f64::MAX);



    // OBJECTS

    // Hey look, we are telling Rust we need to use HashMap from it's standard library
    use std::collections::HashMap;

    // Structs are great for representing data structures
    struct Person {
        name: String,
        age: i16, // Yes, dangling commas are allowed and the convention
    }


    // Using structs is basically like when defining them, but with values
    let fredrik = Person {
        name: "Fredrik".to_string(),
        age: 33,
    };
    // Snake case is so much the convention that the compiler will warn you if you try to use camelCase
    let unknown_person = Person {
        name: "Unknown".to_string(),
        age: 0,
    };

    println!("Hi there {} and {}", fredrik.name, unknown_person.name);

    // Let's create a HashMap, these work more or less as es6 Sets
    // So when you want to hold arbitrary keys with arbitrary values HashMap is your new best friend
    let mut ages = HashMap::new();

    // Insert name as key and age as value into the HashMap
    ages.insert(&fredrik.name, &fredrik.age);
    ages.insert(&unknown_person.name, &unknown_person.age);

    // Print ages to see what we have, notice the {:?} instead of {} here?
    // Complex types need to specify how they should be printed to work with {}
    // {:?} instead makes use of a Debug trait in Rust, that can print
    // almost anything, though not always in a very pretty, readable format
    println!("ages {:?}", ages);

    // We can also remove stuff
    ages.remove(&unknown_person.name);
    println!("ages {:?}", ages);

    // And we can also get stuff of course
    if let Some(fredrik_from_the_ages) = ages.get(&fredrik.name) {
        println!("Fredrik's age is {}", fredrik_from_the_ages);
    }
    // What is this sorcery? Why the if, and what is `Some`?



    // ARRAYS

    // So we first specify the type of values the vector will hold,
    // and then we call new to create an empty vector
    // Also notice the `mut`, without it we can't push or change anything
    let mut fruits: Vec<String> = Vec::new();

    // Now we can push stuff to it
    fruits.push("Banana".to_string());
    fruits.push("Banana".to_string());
    fruits.push("Banana".to_string());
    fruits.push("Orange".to_string());
    fruits.push("Orange".to_string());

    // values can be accessed by index of course
    println!("{} is a fruit", &fruits[0]);

    // for in should feel familiar? will just print all fruits one by one
    for fruit in &fruits {
        println!("{}", fruit);
    }

    // You can also loop over a range of integers like this
    // This will let us print out the lines of that bad joke you probably saw coming
    // When the fruits vector was populated ;)
    for i in 0..fruits.len() {
        // Match is the switch of Rust, it's smarter as you'll learn later,
        // but this might as well be a switch
        match i {
            // {} creates a block so we can do more than one thing here
            // => will expect one expression
            0 => {
                println!("Knock, knock");
                println!("Who's there?");
            },
            1 => {
                println!("{}. Knock, knock", fruits[i]);
                println!("Who's there???");
            },
            2 => {
                println!("{}. Knock, knock", fruits[i]);
                println!("WHO'S THERE???");
            },
            3 => {
                println!("{}", fruits[i]);
                println!("{}, who?", fruits[i]); },
            4 => {
                println!("{} you glad I didn't say {}?", fruits[i], fruits[0]);
                println!("facepalm");
            },
            // Rust wants to make sure your match statements always get a match to avoid
            // unexpected behaviors, `_` is the "default" or "catch all" rule
            _ => println!("You are not even a fruit"),
        }
    }

    // The `vec!` macro is a shorthand for creating vectors
    let nums = vec![1,2,3,4,5];

    // We need to specify the type here to make the compiler happy
    let multiplied: &Vec<i32> = &nums
        // Get an iterator from nums
        .iter()
        // Map over it and multiply eac number by 2
        .map(|num| num * 2)
        // Filter out numbers that got too big for our taste
        .filter(|num| *num < 8)
        // collect the result into a new vector
        .collect();
    println!("Multiplied: {:?}", multiplied);
}
