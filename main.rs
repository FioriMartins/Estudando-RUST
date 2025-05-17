// a seta indica o TIPO do retorno, que no caso é um inteiro de 32 bytes
fn soma(a:i32, b:i32) -> i32 {
    println!("{} + {} = {}", a, b, a + b);
    a + b // sem ponto e virgula para indicar que é o retorno, também da para utilizar o "return"
}

fn main() {
    println!("Soma = {}", soma(2, 2));
}
