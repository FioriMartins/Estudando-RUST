fn main() {
    let nomevariavel:i32 = 128;
    println!("Variavel = {}, tamanho = {} bytes", nomevariavel, std::mem::size_of_val(&nomevariavel));

    let decimal:f32 = 2.5;
    println!("decimal = {}", decimal);

    // variaveis no RUST são imutáveis
    // mas você pode torná-la mutável com 'mut' antes da declaração do nome
    let mut booleana = true;
    booleana = false;
    println!("Valor= {}, Tamanho booleana = {}", booleana, std::mem::size_of_val(&booleana));

    let letra:char = 'C';
    println!("Tamanho do char = {}", std::mem::size_of_val(&letra));
}
