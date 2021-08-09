use std::io;

fn convert_to_int(data_input:& String)-> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn fatorial (num: i32) -> i32{
    let mut mult = 1;
    for i in 1..(num+1){
        mult *= i;
    }
    mult
}


fn main() {
    
    println!("coloque o tamanho da entrada: ");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("temos um erro de leitura da variavel entrada");

    for i in 0..convert_to_int(&entrada){
        println!("entre com um numero da sequencia: ");
        let mut entrada2 = String::new();
        io::stdin().read_line(&mut entrada2).expect("temos um erro de leitura da variavel entrada2");
        println!("o fatorial de  {} eh {}", entrada2, fatorial(convert_to_int(&entrada2)));
    }
    


}