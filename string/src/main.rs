fn main() {
    println!("Hello, world!");

    let mut s = String::from("hello world!");

    println!("The length of '{s}' is {s}");

    s.push_str("you?");

    println!("{s}");

    let x = 5;
    let y = x;


    let s1 = String::from("hello");
    {
        let s2 = s1.clone();
        println!("{s2}");
    }
    println!("{s1}");

    takes_ownership(s1);

    let x = 5;

    makes_copy(x);

    let s1 = String::from("hello world!");
    let (s3,length) = calculate_length(s1);

    println!("{s3}.len() = {length}");


}



fn takes_ownership(s: String) {
    println!("take ownership of '{s}'");
}

fn makes_copy(s: i32) {
    println!("make a copy of '{s}'");
}


fn calculate_length(s: String) -> (String, usize) {
    println!("The length of '{s}' is {s}");
    let len = s.len();
    (s,len)
}
