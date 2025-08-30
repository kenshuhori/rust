#[derive(serde::Serialize, serde::Deserialize)]
struct Person {
    nickname: String,
    age: u8,
}

fn main() {
    let person = Person {
        nickname: String::from("タロー"),
        age: 30,
    };

    let serialized = serde_json::to_string(&person).unwrap();
    let deserialized: Person = serde_json::from_str(&serialized).unwrap();

    assert_eq!(serialized, "{\"nickname\":\"タロー\",\"age\":30}");
    assert_eq!(person.nickname, deserialized.nickname);
    assert_eq!(person.age, deserialized.age);
}
