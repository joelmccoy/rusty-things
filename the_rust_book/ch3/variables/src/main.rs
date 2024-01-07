use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x isn the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("{}, {}, {}", x, y, z);

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    another_function(5);
    
    let y = {
        let x = 3;
        x + 1
    };
    
    println!("The value of y: {y}");
    
    let five = five();

    println!("Five is definitely {five}");
    
    let number = 3;

    if number > 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    

    let number = if true {5} else {6};
    println!("The value of number is: {number}");
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn five() -> u32 {
    5
}