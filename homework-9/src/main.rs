fn main() {
    let x = 3;
    let y = 4;
    println!("{:?}", sum_of_square(x, y))
}

fn square(int: u8) -> u8 {
    int * int
}

fn sum_of_square(a: u8, b: u8) -> u8 {
    let a = square(a);
    let b = square(b);
    a + b
}
