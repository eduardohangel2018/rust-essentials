fn main() {
    implicitly_typed_borrowing();
    explicitly_typed_borrowing();
    mutable_borrowing();
}

// s é o dono (owner) da string s1, enquanto r1 é somente um tipo de observador (observer)
// r é uma referência para esse objeto string
fn implicitly_typed_borrowing() {
    let s1 = String::from("eduardo");
    let r1 = &s1;

    println!("Empréstimo implicitamente tipado: s1: {}, r1: {}", s1, r1);

}

pub fn explicitly_typed_borrowing() {
    let s2: String = String::from("eduardo");
    let r2: &String = &s2;

    println!("Empréstimo explícito tipado s2: {}, r2: {}", s2, r2);
}

// Defini um objeto mutable via keyword mut
fn mutable_borrowing() {
    let mut s: String = String::from("testando");
    // Posso emprestar o valor mutable via sintax &mut
    let r: &mut String = &mut s;

    r.push_str(" som");
    println!("{}", r);
}