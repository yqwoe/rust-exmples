fn main() {
    println!("Hello, world!");
    let number = 3;

    if number < 5 {
        println!("Number is less than 5");
    } else if number < 10 {
        println!("Number is less than 10");
    } else {
        println!("Number is greater than or equal to 10");
    }

    let number = 3;
	let result = if number >= 3 { 10 } else {0};
	println!("The result is: {result}");

    let mut counter = 0;
    let result = loop {
        counter +=1;
        if counter == 99 {
            break counter * 2;
        }
    };
    println!("The result is: {result}");


    let mut  count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop{
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2{
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {count}");



    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("End");


    let a = [10,20,30,40,50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("the value is: {number}");
    }

    // calc();

    let number = fibonacci(12);

    // println!("fibonacci(100) {number}");

    for n in number{
        println!("fibonacci the value is: {n}");
    }

}


use std::{io::stdin};


fn  calc(){
    let mut mode = String::new();

    println!("Type in f2c or c2f");

    stdin().read_line(&mut mode).expect("Failed to read line");

    let rate:f64 = 32.0;

    let mut inputNumber = String::new();



    println!("Type in {mode} number ");

    stdin().read_line(&mut inputNumber).expect("Failed to read line");

    let inputNumber:f64 = match inputNumber.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number");
            0.0
        }
    };

    if mode.trim() == "f2c"{
        let a = inputNumber - rate;
        let result:f64 = a / 1.8;
        println!(" f 2 c result: {result}");
    }else if mode.trim() == "c2f"{
        let a = inputNumber * 1.8;
        let result: f64 = rate + a;
        println!(" c2f result: {result}");
    }

}


/// F(0) = 0
/// F(1) = 1
/// F(n) = F(n-1) + F(n-2)（当n≥3且n∈自然数集N*）
fn fibonacci(mut n: i32) -> Vec<i32> {
    let mut result:Vec<i32> = vec![];
    match n {
        0 => result.push(0),
        1 => result.push(1),
        2 => result.push(1),
        _ => {
            result.append(&mut fibonacci(n - 1));
            result.append(&mut fibonacci(n - 2));
        },
    }
    result
}



