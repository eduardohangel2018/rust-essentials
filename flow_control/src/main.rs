mod mytypes;
use mytypes::Color;


fn demo_break_continue() {
    let array = [99, 45, 85, 100, 82];
    for elem in array {
        if elem == 100 {
            println!("Encontrei, apenas utilizei o break para parar");
            break;
        }

        println!("{}", elem);
    }

    for elem in array {
        if elem < 50 {
            println!("Encontrei o número: {}, que é menor que 50, continue na iteração", elem);
            continue;
        }
        println!("Número maior que 50: {}", elem)
    }
}

fn simple_enums() {
    let c: Color = Color::Green;
    match c {
        Color::Red => println!("Pare"),
        Color::Green => println!("Siga"),
        Color::Yellow => println!("Atenção")
    }
}


fn main() {
    demo_break_continue();
    simple_enums();
}