#[derive(Debug)]
pub enum Error {
    EmptyNickname,
    InvalidAge(i64),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EmptyNickname => write!(f, "Nickname cannot be empty."),
            Error::InvalidAge(age) => write!(f, "You provide {0}, but it must be between {min} and {max}.", age, min = u8::MIN, max = u8::MAX),
        }
    }
}

impl std::error::Error for Error {}

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