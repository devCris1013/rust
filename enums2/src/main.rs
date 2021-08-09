enum Pagamento{
    Dinheiro(f32),
    CreditoCartao(bool,f32),
    _DebitoCartao(bool,f32)
}


fn main() {

    let pessoa_pagamento = Pagamento::CreditoCartao(true,100f32);
    match pessoa_pagamento{
        Pagamento::Dinheiro(f) => println!("pagamento em dinheiro {} reais", f),
        Pagamento::CreditoCartao(true,x) => println!("pagamento em cartao de credito {}", x),
        Pagamento::CreditoCartao(false,x) => println!("pagamento em cartao de credito nao foi aprovado no valor de {}", x),
        _ => {}
    }

}
