fn main() {
    let n: usize = 20;
    fizz_buzz(n);
}

fn fizz_buzz(n: usize) {
    for i in 1..=n {
        match i {
            i if i % 15 == 0 => println!("{} : FizzBuzz", i),
            i if i % 3 == 0 => println!("{} : Fizz", i),
            i if i % 5 == 0 => println!("{} : Buzz", i),
            _ => println!("{}", i),
        }
    }
}