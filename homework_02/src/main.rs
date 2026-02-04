use std::i8;

fn val_zero(tup: (i8, f32)) -> i8 {
    tup.0
}

fn val_one(tup: (i8, f32)) -> i8 {
    tup.1 as i8
}

fn main() {

    let tup = (i8::MAX, f32::MAX);
    let c = val_zero(tup) + val_one(tup);
    println!("{c}");
}
