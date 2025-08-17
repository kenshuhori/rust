#[derive(Debug)]
struct Person {
    nickname: String,
    age: Age,
}

#[derive(Debug)]
struct Age(u8);

impl From<u8> for Age {
    fn from(value: u8) -> Self {
        Age(value)
    }
}

fn main() {
    let age_value = 35_u8;
    let yoshida = Person {
        nickname: String::from("yosshi-"),
        age: age_value.into(),
    };
    println!("{:?}", yoshida);
}
