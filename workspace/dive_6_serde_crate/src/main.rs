use serde::ser::SerializeStruct;

struct Person {
    nickname: String,
    age: u8,
}

impl serde::ser::Serialize for Person {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Person", 2)?;
        state.serialize_field("nickname", &self.nickname)?;
        state.serialize_field("age", &self.age)?;
        state.end()
    }
}

impl<'de> serde::de::Deserialize<'de> for Person {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PersonVisitor;

        impl<'de> serde::de::Visitor<'de> for PersonVisitor {
            type Value = Person;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Person")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Person, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut nickname = None;
                let mut age = None;
                while let Some(key) = map.next_key::<&str>()? {
                    match key {
                        "nickname" => {
                            if nickname.is_some() {
                                return Err(serde::de::Error::duplicate_field("nickname"));
                            }
                            nickname = Some(map.next_value()?);
                        }
                        "age" => {
                            if age.is_some() {
                                return Err(serde::de::Error::duplicate_field("age"));
                            }
                            age = Some(map.next_value()?);
                        }
                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }
                let nickname =
                    nickname.ok_or_else(|| serde::de::Error::missing_field("nickname"))?;
                let age = age.ok_or_else(|| serde::de::Error::missing_field("age"))?;
                Ok(Person { nickname, age })
            }
        }

        deserializer.deserialize_struct("Person", &["nickname", "age"], PersonVisitor)
    }
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
