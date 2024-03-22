#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[derive(Debug)]
struct Color(i32,i32,i32);
#[derive(Debug)]
struct Point(i32,i32, i32);
#[derive(Debug)]
struct AlwaysEqual;

#[derive(Debug)]
struct Square(u32,u32);


fn main() {
    println!("Hello, world!");

    let mut user =  build_user("dong33@163.com","dong33");

    println!("user is {:?}",user);

    let mut user1 = User{
        active: true,
        username: user.username,
        email: user.email,
        sign_in_count: 1,
    };


    println!("user1 is {:?}",user1);
    dbg!(user1);

    let black = Color(0,0,0);


    println!("black is {:?}",black);
    dbg!(black);

    let origin = Point(0,0,0);

    println!("origin is {:?}",origin);
    dbg!(origin);

    let subject = AlwaysEqual;
    println!("subject is {:?}",subject);
    dbg!(subject);


    let width = 30;
    let height = 50;

    println!(" area is {}",area(width, height));


    let rect = Square(30,50);
    println!("rect is {:?}",rect);
    dbg!(&rect);


    println!("square area is {}",area_square(&rect));

}


fn build_user(email: &str,username: &str) -> User {
    User {
        active: true,
        username: String::from(username),
        email: String::from(email),
        sign_in_count: 1,
    }
}


fn area(width: u32,height: u32) -> u32{
    width * height
}


fn area_square(square: &Square) -> u32{
    square.0 * square.1
}
