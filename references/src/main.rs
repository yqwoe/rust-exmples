fn main() {
    println!("Hello, world!");

    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);

    println!("The length of '{}' is {}.", s1, s1.len());
}


fn calculate_length(s: &String) -> usize{
    s.len()
}

fn change(s: &mut String){
    s.push_str(", world!!!!");
}
