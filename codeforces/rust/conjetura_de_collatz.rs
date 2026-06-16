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
    let mut n: i32 = int();                   
    
    while true {
        println!("{}", n);
        if n == 1 {
            break;
        }
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
    }
}