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

impl Into<Age> for u8 {
    fn into(self) -> Age {
        Age(self)
    }
}

fn main() {
    let age_value = 35_u8;
    let yoshida = Person {
        nickname: String::from("yosshi-"),
        age: age_value.into(),
    };
    println!("{} / {}", yoshida.nickname(), yoshida.age().0);
}
