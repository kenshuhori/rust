#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Nickname cannot be empty.")]
    EmptyNickname,
    #[error("You provide {0}, but it must be between {min} and {max}.", min = u8::MIN, max = u8::MAX)]
    InvalidAge(i64),
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