fn main() {
    println!("Fizz buzz 1-100 started.");
    for x in 1..101 {
        if x % 3 == 0 && x % 5 == 0 {
            print!("fizzbuzz ");
        }
        else if x % 3 == 0 {
            print!("fizz ");
        }
        else if x % 5 == 0 {
            print!("buzz ");
        }
        else {
            print!("{} ", x);
        }
    }
    println!("");
}
