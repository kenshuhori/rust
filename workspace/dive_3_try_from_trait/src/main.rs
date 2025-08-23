#[derive(Debug)]
struct Person {
    nickname: String,
    age: Age,
}

impl Person {
    fn nickname(&self) -> &str {
        &self.nickname
    }

    fn age(&self) -> &Age {
        &self.age
    }
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
        age: Age::from(age_value),
    };
    assert_eq!(yoshida.nickname(), "yosshi-");
    assert_eq!(yoshida.age().0, 35);
}
