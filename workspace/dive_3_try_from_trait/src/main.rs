#[derive(Debug)]
pub struct Person {
    pub nickname: String,
    pub age: Age,
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
    let nickname_value = "yosshi-";
    let yoshida = Person {
        nickname: String::from(nickname_value),
        age: Age::try_from(age_value),
    };
    assert_eq!(yoshida.nickname, nickname_value);
    assert_eq!(yoshida.age.0, age_value);

    println!("実行は正常に終了しました");
}
