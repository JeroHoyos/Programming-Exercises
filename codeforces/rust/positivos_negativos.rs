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
    
    let mut negativos = 0;

    let mut positivos = 0;

    for i in 0..n {
        let x: i32 = int();               
        if x < 0 {
            negativos += x;
        } else if x > 0 {
            positivos += x;
        }
    }
    println!("positivos {}, negativos {}", positivos, negativos);
}