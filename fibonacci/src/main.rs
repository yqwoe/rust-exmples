fn main() {
    fibonacci(22);
}


fn fibonacci(n: i32) -> i32 {
    match n {
        0 => 0,
        1 =>1,
        2 => 1,
        _ => fibonacci(n-1) + fibonacci(n-2),
    }
}
