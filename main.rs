const PI:f32 = 3.14; // As constantes não possuem um endereço de memória
                     // isso significa: que const são inlined em tempo de compilação, ou seja, o
                     // compilador substitui a referência para a constante pelo  seu valor
                     // diretamente
static mut GLOBAL:u8 = 1;

fn main() {
    println!("PI = {}", PI);

    // sem o unsafe, o RUST iria avisar que variável global é 'insegura'
    // com unsafe, de uma forma simplificada estamos dizendo que sabemos que é inseguro.
    unsafe {
        println!("Variável Global = {}", GLOBAL);
    }

    let nomevariavel:i32 = 128;
    println!("Variável = {}, tamanho = {} bytes", nomevariavel, std::mem::size_of_val(&nomevariavel));

    let decimal:f32 = 2.5;
    println!("Decimal = {}", decimal);

    let booleana = true;
    println!("Valor= {}, Tamanho booleana = {}", booleana, std::mem::size_of_val(&booleana));

    let letra:char = 'C';
    println!("Tamanho do char = {}", std::mem::size_of_val(&letra));
}
