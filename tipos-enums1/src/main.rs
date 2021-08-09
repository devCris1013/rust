#[derive(Debug)]
enum CarType{
    Fiat,
    Ford,
    Renault
}

fn nacionalidade_carro(car:CarType){
    match car{
        CarType::Fiat => println!("O carro eh italiano!"),
        CarType::Ford => println!("o carro e americano!"),
        CarType::Renault => println!("o carro eh frances!")
    }
}

fn main() {
    nacionalidade_carro(CarType::Fiat);
    nacionalidade_carro(CarType::Ford);
    nacionalidade_carro(CarType::Renault);
    
}
