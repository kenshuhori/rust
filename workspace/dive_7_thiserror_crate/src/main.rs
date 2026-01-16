mod person;

use person::Person;

fn main() {
    let person_with_invalid_nickname = Person::new(
        "",
        30,
    );

    match person_with_invalid_nickname {
        Ok(_) => {
            println!("Person created successfully.");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    let person_with_invalid_age = Person::new(
        "Alice",
        -50,
    );

    match person_with_invalid_age {
        Err(e) => {
            println!("Error: {}", e);
        }
        Ok(_) => {
            println!("Person created successfully.");
        }
    }
}