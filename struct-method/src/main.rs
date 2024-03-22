#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

  	fn can_hold(&self,target: &Rectangle ) -> bool {
      self.width > target.width && self.height > target.height
    }

    fn square(size: u32) -> Self{
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle::square(30);

    let rect1 = Rectangle { width: 10, height: 30 };

    let rect2 = Rectangle { width: 60, height: 80 };
    dbg!(&rect);
    println!("The area of the rectangle is {} square pixels.", rect.area());
  	println!("Can rect hold rect1? {}", rect.can_hold(&rect1));
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
}

