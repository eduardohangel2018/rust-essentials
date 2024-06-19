pub struct Person {
    pub name: String,
    pub age: i32,
}

impl Person{
    pub fn new(name: String, age: i32) -> Person {
        Person {name, age}
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_age(&self) -> i32 {
        self.age
    }

    pub fn verify_age(&self) {
        if self.age > 70 {
            println!("Você deve ter cuidado ao dirigir");
        }
        else if self.age > 18 {
            println!("Você pode dirigir")
        }
        else {
            println!("Você é novo demais para dirigir")
        }
    }

    pub fn if_expression(&self) {
        let msg = if self.age > 70 {"velho"} else {"novo"};
        println!("Você é {}", msg);
    }

}