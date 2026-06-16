use std::io::stdin;

fn line() -> String {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn int() -> i32 {
    line().parse().unwrap()
}

fn ints() -> Vec<i32> {
    line().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let n: i32 = int();                   
    
    if n % 2 == 0 {
        println!("es multiplo de 2");
    } else 
    if  n % 3 == 0 {
        println!("es multiplo de 3");
    } else 
    if n % 5 == 0 {
        println!("es multiplo de 5");
    } else 
    if n % 7 == 0 {
        println!("es multiplo de 7");
    } else {
        println!("no es multiplo de ninguno de los primeros cuatro primos");
    }
}