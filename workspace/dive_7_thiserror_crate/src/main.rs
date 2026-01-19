mod person;
mod person_with_std_error;

// use person::Person;
// use person_with_std_error::Person;

fn main() {
    let person_with_invalid_nickname = person_with_std_error::Person::new(
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

    let person_with_invalid_age = person_with_std_error::Person::new(
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