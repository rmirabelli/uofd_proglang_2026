fn main() {
    let x = 5;
    println!("The value of x is {}", x);
    let x = x + 1;
    println!("The value of x is {}", x);
    let x = x * 2;
    println!("The value of x is {}", x);
}

fn change_type() {
    let spaces = "   ";
    let spaces = spaces.len();

    let mut spaces = "     ";
    spaces = spaces.len();
}

fn tuples() {
    let tup: (i32, f64, u8) = (100, 1.1, 6);
}

fn tuple_destructuring() {
    let tup = (100, 1.1, 6);
    let (x, y, z) = tup;
    println!("The value of y is {}", y);
}

fn tuple_direct_access() {
    let tup = (100, 1.1, 6);
    let y = tup.1;
    println!("The value of y is {}", y);
}

fn emoji() {
    let a = 'a';
    let a = '𝒜';
    let a = '🧚';
}

fn add_types() {
    let x:u32 = 5;
    let y:f32 = 20.0;
    let z = x + y;
}

fn holder() {

    let fm = usize::MAX;
}