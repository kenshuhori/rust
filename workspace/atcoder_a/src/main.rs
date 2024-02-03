use proconio::input;

fn main() {
    input! {
        value: String,
    }
    let num: i32 = match value.parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            println!("error");
            return;
        }
    };
    println!("{}", num * 2);
}
