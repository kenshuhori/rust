#[derive(Debug)]
pub struct Person {
    nickname: String,
    age: Age,
}

#[derive(Debug)]
struct Age(u8);

#[derive(Debug)]
enum AgeError {
    Impossible,
}

impl TryFrom<u8> for Age {
    type Error = AgeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 130 {
            return Err(AgeError::Impossible);
        }
        Ok(Age(value))
    }
}

fn main() {
    let age_value = 135_u8;
    let nickname_value = "yosshi-";
    let yoshida = Person {
        nickname: String::from(nickname_value),
        age: Age::try_from(age_value).expect("年齢の変換は成功するはず"),
    };
    assert_eq!(yoshida.nickname, nickname_value);
    assert_eq!(yoshida.age.0, age_value);

    println!("実行は正常に終了しました");
}
