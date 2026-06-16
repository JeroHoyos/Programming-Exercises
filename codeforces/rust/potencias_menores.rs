use std::io::stdin;

fn line() -> String {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn int() -> i64 {
    line().parse().unwrap()
}

fn ints() -> Vec<i64> {
    line().split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let a: i64 = int();                   
    let mut exponent = 1;
    let b: i64 = int();
    
    while true {
        if a.pow(exponent) <= b {
            println!("{}", a.pow(exponent));
            exponent += 1;
        } else {
            break;
        }
    }
}