
#[derive(Debug)]
struct Person {
    name: String,
    age: i32
}

#[derive(Debug)]
struct Location(f32, f32);

fn main() {
    let david = Person { name: String::from("David Lebee"), age: 34 };

    let myHome = Location(12312.233232, 2132132.323);
    println!("{:?}", myHome);    
    println!("{:?}", david);

    if david.age > 18 {
        println!("{} is an adult", david.name);
    }
}