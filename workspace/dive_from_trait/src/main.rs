#[derive(Debug)]
struct Person {
    nickname: String,
    age: Age,
    height: u8,
    weight: u8,
}

#[derive(Debug)]
struct Age(u8);

fn main() {
    let age_value = 35_u8;
    let yoshida = Person {
        nickname: String::from("yosshi-"),
        age: Age::from(age_value),
        height: 170,
        weight: 65,
    };
    println!("{:?}", yoshida);
}
