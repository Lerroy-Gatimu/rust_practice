use std::collections::HashMap;

pub fn export() {
    println!("Heyyyy");
}

pub fn arrays_practice() {
    let mut a = vec![1, 2, 3];
    a.push(4);
    a[0] = 0;
    a.pop();
    a.insert(1, 5);
    a.remove(3);
    println!("{:?}", a);
}

/*
Note: Use &fruits to borrow the vector instead of moving it.
In Rust, borrowing means using a reference to a value instead of taking ownership of it. When you loop through a vector without &, the values are moved out, and you can no longer use the vector. But when you borrow the vector using &, you can still use it later in your program.
*/

pub fn vector_borrowing() {
    let mut a = vec![1, 2, 3, 4, 5];
    a.pop();
    for _number in &a {
        println!("There are {} numbers", a.len());
    }
}

//Tuples
//When we create a tuple, we normally assign values to it. This is called "packing" a tuple
//But, in Rust, we are also allowed to extract the values back into variables. This is called "unpacking"

pub fn tuples_prac() {
    let a = ("Lerroy", 23, true);
    let (name, age, active) = a;

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Active: {}", active);

    //Return a Tuple from a Function
    //Tuples are often used to return multiple values from a function:

    fn tuple1() -> (String, i32) {
        (String::from("Lerroy"), 25)
    }

    fn call_tuple() {
        let user = tuple1();
        println!("Name: {}, Age: {}", user.0, user.1)
    }

    tuple1();
    call_tuple();
}

pub fn hashmaps() {
    let mut capital_cities = HashMap::new();
    capital_cities.insert("China", "Beijing");
    capital_cities.insert("Russia", "Moscow");
    capital_cities.insert("UAE", "Abu Dhabi");
    let c1 = &capital_cities.get("China");
    println!("{:?}", capital_cities);
    println!("{:?}", c1);
    for (country, city) in capital_cities {
        println!("The capital of {} is {}", country, city)
    }
}

//Update Values
//If you insert a new value using a key that already exists, the old value is replaced with the new one

//To remove a key from a HashMap, use the .remove() method

/*
Structs
A struct (short for "structure") is a custom data structure that lets you group related values together.
You can think of a struct like a mini-database for one thing, like a person with a name and age.
*/

pub fn structs() {
    struct Person {
        name: String,
        age: u32,
        can_vote: bool,
    }
    let user1 = Person {
        name: String::from("Lerroy"),
        age: 23,
        can_vote: true,
    };
    println!("Name: {}", user1.name);
    println!("Age: {}", user1.age);
    println!("Voter eligibility: {}", user1.can_vote);
}

pub fn enums() {
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    fn directions_match() {
        let my_direction = Direction::Left;

        match my_direction {
            Direction::Up => println!("Going up"),
            Direction::Down => println!("Going down"),
            Direction::Left => println!("Going left"),
            Direction::Right => println!("Going right"),
        }
    }
    directions_match();
}
