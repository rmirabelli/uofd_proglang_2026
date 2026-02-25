enum Composition {
    Solo,
    Duet,
    Trio,
    Quartet,
    Combo(u32)
}

fn main() {
    let group = Composition::Combo(42);

    if let Composition::Combo(val) = group {
        println!("there are {} members", val);
    } else {
        println!("a small group");
    }
}
