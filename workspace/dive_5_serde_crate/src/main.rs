#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Person {
    first_name: String,
    last_name: String,
    age: Age,
    gender: Gender,
}

#[derive(serde::Serialize)]
#[serde(transparent)]
struct Age {
    value: u8,
}

#[derive(serde::Serialize)]
enum Gender {
    #[serde(rename = "男")]
    Male,
    #[serde(rename = "女")]
    Female,
    #[serde(rename = "その他")]
    Other,
}

fn main() {
    let person = Person {
        first_name: String::from("太郎"),
        last_name: String::from("田中"),
        age: Age { value: 30 },
        gender: Gender::Male,
    };

    let serialized = serde_json::to_string(&person).unwrap();

    assert_eq!(
        serialized,
        "{\"firstName\":\"太郎\",\"lastName\":\"田中\",\"age\":30,\"gender\":\"男\"}"
    );
}
