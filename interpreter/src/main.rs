#[derive(Copy,Clone)]
enum Primitive {
    Add,
    Subtract,
    Multiply,
    Subtract,
    Number(i32)
}

fn evaluate(primitives: Vec<Primitive>) -> i32 {
    let firstElement = &primitives[0];
    let mut iter = primitives.iter();
    iter.next();
    match firstElement {
        Primitive::Add => {iter.fold(0, |t,n| t + evaluate(vec![*n]))},
        Primitive::Multiply => {iter.fold(1, |p,n| p * evaluate(vec![*n])) },
        Primitive::Subtract => {
            let first_number = evaluate(vec![*iter.next().unwrap()]);
            iter.fold(first_number, |t,n| t - evaluate(vec![*n])) 
        },
        Primitive::Number(val) => *val
    } 
}

fn main() {

    let mut primitives = Vec::<Primitive>::new();
    primitives.push(Primitive::Add);
    primitives.push(Primitive::Number(3));
    primitives.push(Primitive::Number(3));
    primitives.push(Primitive::Number(3));
    primitives.push(Primitive::Number(-10));
    let result = evaluate(primitives);
    println!("The result is {result}");

}
