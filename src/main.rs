mod minor;

fn simple_math() {
    let add = 5 + 3;
    let sub = 10 - 4;
    let mul = 6 * 2;
    let div = 12 / 3;
    let rem = 10 % 3;

    println!("Add: {}", add);
    println!("Sub: {}", sub);
    println!("Mul: {}", mul);
    println!("Div: {}", div);
    println!("Rem: {}", rem);
}

fn comparison() {
    let a: i32 = 30;
    const BA: i32 = 20;

    println!("Is {} greater than {}? {}", a, BA, a > BA);
}

fn user1() {
    let is_user: bool = true;
    let is_admin: bool = false;
    let is_staff: bool = true;

    println!("Is regular user: {}", is_user && !is_staff);
    println!("Is staff: {}", is_staff || is_admin);
}

fn score() {
    let score: i32 = 60;
    if score >= 80 {
        println!("Grade: A")
    } else if score >= 70 {
        println!("Grade: B")
    } else {
        println!("You did not do well")
    }
}

fn days() {
    let day: i32 = 6;
    match day {
        1 => println!("Sunday"),
        2 => println!("Tuesday"),
        3 => println!("Thursday"),
        _ => println!("Invalid day"),
    }
}

fn loops() {
    let mut count = 1;
    loop {
        println!("Suppp");
        if count == 3 {
            break;
        }
        count += 1
    }
}

fn while_loop() {
    let mut a: i32 = 30;

    while a > 2 {
        if a == 10 {
            a -= 1;
            continue;
        }
        if a > 2 {
            if a == 15 {
                break;
            }
        }

        a -= 1;
    }

    println!("The counter has reached: {}", a);
}

fn for_loops() {
    for i in 1..3 {
        //If you want to include the last number, use ..= (two dots and an equals sign):
        println!("Hey {}", i);
    }

    for b in 1..=10 {
        if b == 3 {
            continue;
        }
        if b == 7 {
            break;
        }
        println!("The loop is at {}", b)
    }
}

fn func_param(name: &str) {
    println!("Hello {}", name);

    fn add(a: i32, b: i32) -> i32 {
        return a + b;
    }
    let result = add(5, 6);
    println!("Sum {}", result)
}

fn strings() {
    let mut text1 = "Hello Mr. ".to_string();
    let mut text2 = String::from("Lerrize");
    text1.push_str("How is your day? ");
    text2.push('!');
    let result = format!("{}{}", text1, text2);
    println!("{}", result);

    /*You can also use the + operator to combine strings, but it can get messy with many values.

    Example
    let s1 = String::from("Hello");
    let s2 = String::from("World!");
    let s3 = String::from("What a beautiful day!");
    let result = s1 + " " + &s2 + " " + &s3;
    println!("{}", result);
    Note: You can only add a &str to a String with +. That is why &s2 and &s3 (a string slice) is used here.*/
}

fn ownership() {
    let a = String::from("Hey");
    let b = a;
    println!("{}", b);
    /*When we assign a to b, the ownership moves. This means only b can use the value now, because a is no longer valid.

    But simple types like numbers, characters and booleans are copied, not moved.
    let a = 5;
    let b = a;
    println!("a = {}", a);  // Works
    println!("b = {}", b);  // Works

    For other types, like String, if you really want to keep the original value and also assign it to another variable, you can use the .clone() method, which makes a copy of the data:
    */
    let ab = String::from("Grazieee");
    let ba = ab.clone();
    println!("{} {}", ab, ba);
}

fn borrowing() {
    /*
    However, if you don't need to own the value twice, using a reference (&) is usually better than cloning
    Sometimes you want to use a value without taking ownership of it.

    Rust lets you do this using a reference - this is called borrowing
    */
    let abc = String::from("Hello Master Lerrize");
    let cba = &abc;

    println!("abc = {}", abc);
    println!("cba = {}", cba);

    //If you want to change a value through a reference, you need to make the reference mut
    //Note: You can only have one mutable reference to a value at a time!
    let mut a = String::from("Suppp");
    let b = &mut a;
    b.push_str("...Awesome dude");
    println!("{}", b);
}

fn import() {
    minor::export();
}

fn dsa() {
    //An array in Rust is a fixed-size list of values, all of the same type. You cannot grow or shrink an array after it's created.
    let fruits = ["apple", "banana", "orange"];
    println!("Last fruit: {}", fruits[2]);

    //vectors are resizable arrays
    let mut a = vec!["1", "2"];
    a.push("3");
    println!("Third item: {}", a[2])
}

fn main() {
    let name: &str = "Lerrize";
    const BIRTH_YEAR: i32 = 2003;

    println!("Hello, I'm {}, born in {}", name, BIRTH_YEAR);

    simple_math();
    comparison();
    user1();
    score();
    days();
    loops();
    while_loop();
    for_loops();
    func_param("Master");
    strings();
    ownership();
    borrowing();
    import();
    dsa();
}
