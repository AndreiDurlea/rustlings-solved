struct Person {
    name: &'static str,
    age: u32,
    city: &'static str,
}

fn main() {
    let name = "Ana";
    let age = 28;

    // `name` and `age` use the field init shorthand.
    let person = Person {
        name,
        age,
        city: "Bucharest",
    };

    println!("{} ({}) lives in {}", person.name, person.age, person.city);
    assert_eq!(person.city, "Bucharest");
}
