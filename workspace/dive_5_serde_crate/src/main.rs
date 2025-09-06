#[derive(serde::Serialize, serde::Deserialize)]
struct Person {
    name: Name,
    age: Age,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
struct Name {
    first_name: String,
    last_name: String,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
struct Age(u8);

fn main() {
    let person = Person {
        name: Name {
            first_name: String::from("太郎"),
            last_name: String::from("田中"),
        },
        age: Age(30),
    };

    let serialized = serde_json::to_string(&person).unwrap();
    let deserialized: Person = serde_json::from_str(&serialized).unwrap();

    assert_eq!(
        serialized,
        "{\"name\":{\"first_name\":\"太郎\",\"last_name\":\"田中\"},\"age\":30}"
    );
    assert_eq!(person.name, deserialized.name);
    assert_eq!(person.age, deserialized.age);
}
