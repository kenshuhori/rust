#[derive(Debug)]
struct Person {
    nickname: String,
    age: Age,
}

#[derive(Debug)]
struct Age(u8);

fn main() {
    let age_value = 35_u8;
    let yoshida = Person {
        nickname: String::from("yosshi-"),
        age: Age(age_value),
    };
    println!("{:?}", yoshida);
}
