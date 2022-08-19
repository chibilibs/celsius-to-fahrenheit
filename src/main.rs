use std::io;

fn main() {
    println!("*--- Celsius para Fahrenheit ---*");

    println!("Digite a temperatura em Celsius");
    let mut temperatura_celsius = String::new();

    io::stdin().read_line(&mut temperatura_celsius)
        .expect("Falha ao ler entrada");

    let temperatura_celsius: f32 = temperatura_celsius.trim().parse()
        .expect("Tem que ser um Numtero :< ");

    let temperatura_final = (temperatura_celsius * 9.0/5.0) + 32.0;

    println!("\ntemperatura em celsius : {}", temperatura_celsius);
    println!("\nTemperatura em fahrenheit : {}\n", temperatura_final);
}
