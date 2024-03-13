fn main() {
    println!("Hello, world!");
    let xx = another_function(5);

    println!("I am main: {xx}");
}


fn another_function(x: i32) -> i32 {
    println!("I am another function: {x}");
    // let y = (let x + 1);

    let y = {
        let b =  1;
        b + 1
    };


    println!("I am another function: {y}");

    5
}



