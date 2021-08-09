use std::io;
fn convet_to_init(data_input: & String) -> i32{
        let x = data_input.trim().parse::<i32>().unwrap();
        x
}


fn main() {
    let mut estudo = String::new();
    io::stdin().read_line(&mut estudo).expect("nao foi possivel ler a variavel estudo");
    let mut livro1 = 0;
    let mut livro2 = 0;
    while convet_to_init(&estudo) > livro2{
        let mut estudo_livro = String::new();
        io::stdin().read_line(&mut estudo_livro).expect("erro ao ler estudo aluno");
        livro2 += 1;
        if convet_to_init(&estudo_livro) >= 3 && convet_to_init(&estudo_livro) < 6 {
            livro1 += 1;
        }
    }
    println!("quantos alunos fico em recuperação!  {}", livro1);

}

