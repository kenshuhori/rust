#[derive(serde::Serialize)]
struct Person {
    first_name: String,
    last_name: String,
    age: Age,
    gender: Gender,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
struct Age(u8);

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
enum Gender {
    Male,
    Female,
    Other,
}

fn main() {
    let person = Person {
        first_name: String::from("太郎"),
        last_name: String::from("田中"),
        age: Age(30),
        gender: Gender::Male,
    };

    let serialized = serde_json::to_string(&person).unwrap();

    assert_eq!(
        serialized,
        "{\"first_name\":\"太郎\",\"last_name\":\"田中\",\"age\":30,\"gender\":\"Male\"}"
    );
}
