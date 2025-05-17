const PI:f32 = 3.14;
static mut GLOBAL:u8 = 1;

fn sombra() {
    let a = 123;

    {
        let b = 456;
        let a = 457; // aqui não estou redeclarando a variável, estou declarando uma varíavel
                     // totalmente nova, pois a var 'a' não está no mesmo escopo, por mais que o
                     // escopo de dentro herde tudo que o escopo de fora tem.
                     //
                     // o nome dessa característica é "shadowing", pode ser perigoso e vantajoso.

        println!("Dentro, b = {}", b);
    }
    // println!("Fora, b = {}", b)

    println!("Valor de A = {}", a);
}

fn escopo() {
    println!("PI = {}", PI);

    let essa_string = "meu nome"; // Strings geralmente são um vetor ou alguma outra estrutura de
                                  // dados, uma lista, de carecteres. Então é um conjunto de
                                  // caracteres. Isso é uma String. Só que em RUST não posso pegar
                                  // uma string qualquer, por exemplo essa, não posso pegar dessa
                                  // string a primeira posição. Não é realmente um vetor.
                                  // Isso é um tipo mais complexo. Quando crio uma string ela é na
                                  // verdade uma referência para um "static str". E "str" é um
                                  // pedaço de uam string. Ou seja, um pedaço de um vetor de
                                  // caracteres.
    
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

fn main() {
    escopo();
    sombra();
}
