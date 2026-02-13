#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("指定した名前が空です")]
    EmptyName,
    #[error("指定した年齢は{0}ですが{min}から{max}の間でなければなりません。", min = u8::MIN, max = u8::MAX)]
    InvalidAge(i64),
}

pub struct Person {
    pub name: String,
    pub age: u8,
}

impl Person {
    pub fn new(name: &str, age: i64) -> Result<Self, Error> {
        if name.len() == 0 {
            return Err(Error::EmptyName);
        }
        let age_u8 = match u8::try_from(age) {
            Ok(age) => age,
            Err(_) => return Err(Error::InvalidAge(age)),
        };

        Ok(Self {
            name: String::from(name),
            age: age_u8,
        })
    }
}

fn main() {
    let person_with_invalid_name = Person::new(
        "田中 太郎",
        -1,
    );

    match person_with_invalid_name {
        Ok(_) => {
            println!("Person created successfully.");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}