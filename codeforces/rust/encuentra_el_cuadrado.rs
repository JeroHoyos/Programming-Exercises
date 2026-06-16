use std::io;

fn main() {
    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Error al leer");

    let numero: i32 = entrada.trim().parse().expect("No es un número válido");

    println!("{}", numero * numero);
}