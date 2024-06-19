mod flow;
use flow::Person;

fn main() {
    demo_integers();
    float_types();
    bool_types();
    convert_type();
    time_compile();
    redeclare_variable();
    
    let men = Person::new(String::from("Felipe"), 32);
    let girl = Person::new(String::from("Marta"), 20);
    men.get_age();
    men.get_name();
    men.verify_age();
    men.if_expression();
    girl.verify_age();
    for_in_loop();
    demo_loops();
}

fn demo_integers() {
    // Rust possui tipos: i8, i16 etc.
    let a1: i32 = -12345;
    let a2: i32 = 0xFFFF;
    let a3: i32 = 0o177;
    let a4: i32 = 0b1110;

    // Rust possui tipos inteiros não assinados
    let b: u32 = 12345;
    // Rust possui arquitetura específica com tipos isize e usize
    let c: isize = 24680;

    println!("\n Os números são:{} {} {} {} {} {}", a1, a2, a3, a4, b, c);

    // sizeof é uma função genérica que retorna o tipo em bytes
    println!(
        "O isize representa {} bytes na minha máquina",
        std::mem::size_of::<isize>()
    );
}

fn float_types() {
    let f1: f32 = 1.2345;
    println!(
        "Número float com duas casas decimais depois da vírgula: {:.2}",
        f1
    );
}

// Posso converter um bool em um valor de 0 ou 1
fn bool_types() {
    let is_brazilian = true;
    let is_br_number: i32 = is_brazilian as i32;
    println!("Variavel como numero: {}", is_br_number);
}

// Posso converter um tipo utilizando o `as`
fn convert_type() {
    let g = 3.99;
    let h = g as i32;

    println!("f64 é igual a {}", g);
    println!("o mesmo número como i32 é igual a {}", h);
}

// Posso definir constantes em tempo de execução
fn time_compile() {
    const SECONDS_IN_HOUR: i32 = 3_600;
    const SECONDS_IN_DAY: i32 = SECONDS_IN_HOUR * 24;
    println!("Existem {} segundos em um dia.", SECONDS_IN_DAY);
}

// Rust permite redeclarar variáveis em tempo de execução no escopo presente
fn redeclare_variable() {
    let num = "12";
    println!("Número como String: {}", num);

    let num = 12;
    println!("O número + 1 é {}", num + 1);
}


fn for_in_loop() {
    for i in 0..5 {
        println!("{}", i);
    }
}

fn demo_loops() {
    // loop {
    //     println!("Essa mensagem será gerada para sempre, pressione ctr C para parar!");
    //     std::thread::sleep(std::time::Duration::from_secs(1));
    // } 

    let mut i = 0;
    while i <= 5{
        println!("{}", i);
        i +=1;
    }
}
