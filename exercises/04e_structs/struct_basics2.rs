struct Person {
    name: &'static str,
    age: u32,
    city: &'static str,
}

fn main() {
    let name = "Ana";
    let age = 28;

    // TODO: Fix the compiler error. Ana lives in "Bucharest".
    let person = Person { name, age };

    println!("{} ({}) lives in {}", person.name, person.age, person.city);
    assert_eq!(person.city, "Bucharest");
}
