fn main() {
    // println!("Hello, world!");

    let  x = 5;
    println!("the value of x is: {}", x);
    let x = x + 1 ;

    {
        let x = x * 2;
        println!("the value of x inthe inner scope is: {x}")
    }

    println!("the value of x is: {}", x);



    let spaces = "     ";
    let spaces = spaces.len();


    let guess: u32 = "42".parse().expect("Not a number");

    let number: u8 = 255;

    println!("the value of number is: {number}");

    let x = 2.0;
    let y: f32 = 3.0;


    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let truncated = -5 /3 ;

    let remainder = 43 % 5;

    let t = true;


    let tup = (100,20.1,"32.22");

    // let (x,y,z) = tup;

    let x = tup.0;
    let y = tup.1;
    let z = tup.2;

    println!("the value of x is: {x},y is :{y},z is: {z}");

    let a = [1,2,3,4,5,10,10];

    another_function();
}


fn another_function() {
    println!("I am another function");
}
