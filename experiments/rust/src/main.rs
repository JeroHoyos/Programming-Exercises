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
    
    let valores: Vec<i32> = ints(); 
    
    let mut acumulados: Vec<i32> = vec![0; n as usize];
    
    for i in 0..n {

        let x = valores[(n as usize) - 1 - (i as usize)];           
        
        if i == 0 {
            acumulados[i as usize] = x;
        } else {
            acumulados[i as usize] = acumulados[(i - 1) as usize] + x;
        }
    }

    for i in 1..n {
        print!("{}\n", acumulados[i as usize]);
    }
}