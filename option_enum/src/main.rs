fn sec_of_day(hour: u32, min: u32, sec: u32) -> Option<u32> {
    if hour <= 23 && min <= 59 && sec <= 59 {
        let secs = hour * 3600 + min * 60 +sec;
        return Option::Some(secs)
    } 
    else {
        return Option::None
    }
}

// Utilizando o match para testar se a variável option contém um valor válido e extrair o valor

fn option_enum() {
    let seconds: Option<u32>;
    // Valor válido
    seconds = sec_of_day(23,59, 59);

    // Valor inválido
    // seconds = sec_of_day(1234, 567, 8910);

    match seconds {
        Some(s) => println!("Segundos por dia: {}", s),
        None => println!("Segundos por dia: Valor passado inválido")
    }
}


fn main() {
    option_enum();
}