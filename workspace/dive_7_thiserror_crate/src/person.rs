#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid age is provided. You provide {0}, but it must be between 0 and 255.")]
    InvalidAge(i64),
    #[error("Nickname cannot be empty.")]
    EmptyNickname,
}

pub struct Person {
    pub nickname: String,
    pub age: u8,
}

impl Person {
    pub fn new(nickname: &str, age: i64) -> Result<Self, Error> {
        if nickname.len() == 0 {
            return Err(Error::EmptyNickname);
        }
        let age_u8 = u8::try_from(age).map_err(|_| Error::InvalidAge(age))?;

        Ok(Self {
            nickname: String::from(nickname),
            age: age_u8,
        })
    }
}