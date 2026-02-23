#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {

    let r = Rectangle{
        height: 50,
        width: 30
    };

    let s = Rectangle::square(2);

    println!("The {}", r.area());

}
