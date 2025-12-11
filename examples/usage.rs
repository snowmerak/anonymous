use structx::anon_struct;

fn main() {
    let person = anon_struct! {
        derive(Debug);
        name: String = "Alice".to_string(),
        age: u32 = 30,
    };
    println!("{:?}", person);

    let another_person = anon_struct! {
        derive(Debug);
        title: String = "Engineer".to_string(),
        years_experience: u8 = 5,
    };
    println!("{:?}", another_person);

    use std::any::Any;
    let person_type = (&person as &dyn Any).type_id();
    let another_type = (&another_person as &dyn Any).type_id();
    println!("Person type ID: {:?}", person_type);
    println!("Another person type ID: {:?}", another_type);
    println!("Are types equal? {}", person_type == another_type);
}
