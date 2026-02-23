fn main() {

    // FIZZBUZZ!

    for index in 1..=100 {

        let mut found = false;

        if index % 3 == 0 {
            print!{"fizz"};
            found = true;
        }
        if index % 5 == 0 {
            print!{"buzz"};
            found = true;
        }

        if !found {
            print!("{index}");
        }
        print!("\n");
    }

}
