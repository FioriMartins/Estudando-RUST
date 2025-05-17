fn main() {
    let nomevariavel:i32 = 128;
    println!("Variavel = {}, tamanho = {} bytes", nomevariavel, std::mem::size_of_val(&nomevariavel));

    let decimal = 2.5;
    println!("decimal = {}", decimal);
}
