use std::{io, collections::HashMap};

pub fn entry_point()
{
    let opcao = 8;
    match opcao {
        //Testes do Amadeu
        1 => teste_print(),
        2 => sombreamento(),
        3 => tipos(),
        4 => funcao(),
        5 => expressoes(),
        6 => loops(),
        7 => teste_controle_de_memoria(),
        8 => teste_controle_de_memoria_referencia(),
        _ => println!("Opcao invalida...")
    }

}

fn teste_controle_de_memoria_referencia()
{
    
}
// similar ao & do c, manda uma referencia para o dado como parametro, quando acaba o escopo desfaz a referencia
fn controle_de_memoria_referencia(dinamico:&String, mut estatico1:u32, mut estatico2:&u32 )
{
    println!("Inicio da função: dinamico {} estatico1 {} estatico2 {}", dinamico, estatico1, estatico2);
    estatico1=estatico1 + 1;
    println!("estatico1 na função apos mudar {}", estatico1);
    let dado_sem_copy2:&String = dinamico;
    let dado_sem_copy3:&String = dinamico;
    println!("{} {}", dado_sem_copy2,dado_sem_copy3);
    estatico2 = &50;
    println!("estatico2 na função apos mudar {}", estatico2);

}

fn teste_controle_de_memoria()
{
    // stack x heap = mesmo esquema conceitualmente
    // dado dinamico = (alocado na heap, são os dados que geralmente são dados new ou alocados dinamicamente)
    // dado estatico (alocados na stack dados de tamanho conhecido)
    // variavels independente de qual tipo seja são desalocadas apos finalizar o escopo

    // quando atribuimos um dado dinamico a outra variavel, o dado original deixa de existir
    let dinamico1:String = String::from("abc");
    let dinamico2 = dinamico1; // atribuindo o dado dinamico a outra variavel
    println!("dinamico2 eh {dinamico2}");
   // println!("dinamico1 eh {dinamico1}"); => Erro pois "demos" o valor1 para valor2 logo valor1 para de existir
    
    // o mesmo não ocorre com dado estatico
    let estatico1 = 10;
    let estatico2 = estatico1;
    println!("estatico2 eh {estatico2}");
    println!("estatico1 eh {estatico1}");

    // quando passamos um dado dinamico ele vai para função e se torna INUTILIZAVEL na função pai
    // quando passamos o dado estatico ele vai e continua existindo no pai
    // dado dinamico = não implementa copy por isso a função pai perde esse valor
    // dado estatico = implementa copy, ou seja passa uma copia pra função e mantem ele 
    let dinamico:String = String::from("abc");
    let mut estatico:u32 = 10;
    estatico = estatico +1;
    controle_de_memoria(dinamico, estatico);
    println!("estatico fora da função controle_de_memoria eh {estatico}"); // valor não mudou
    // println!("{}", dinamico); =>  erro pois value ja foi desalocado na função acima

    let texto = "abc";
    controle_de_memoria_2(texto);
    println!("Texto depois da func eh {}", texto);
    // note que o tipo de texto é str, não é o mesmo de "dinamico", texto com str são texto com tamanhos conhecidos
    // não adianta usar um valor mutavel para texto, ela continua um dado de tamanho estatico ou seja sempre sera abc
    // ele cai nas regras dos dados estaticos
}

fn controle_de_memoria_2(texto:&str)
{
    println!("Texto na func eh {}",texto);
}
// variavel que implementa copy são dados que vao para a pilha - ou seja não é desalocado quando acaba o scopo
// variaveis que nao implementam copy vai pra heap - é desalocado quando acaba o escopo
fn controle_de_memoria(dinamico_sem_copy:String, mut estatico_com_copy:u32 )
{
    println!("dinamico_sem_copy {} estatico_com_copy {}", dinamico_sem_copy, estatico_com_copy);
    {
        let value_string:String = String::from("a");
        println!("value_string eh {}", value_string);
    }
   // println!("{}", value_string); - erro pos variaveis da heap são desalocadas apos acabar o escopo
   estatico_com_copy = estatico_com_copy + 40;
   println!("estatico_com_copy dentro da função controle_de_memoria é {}",estatico_com_copy);
}
fn loops()
{
    let mut i= 0;

    let x = loop {
        i = i+1;
        if i == 10
        {
            break i+1;
        }
    };
    println!("o loop pode ter um retorno, o valor ao lado do break {}",x);

    i = 0;
    while i != 10 
    {
        i = i+1;
        
    };
    println!("{}",i);

    // 1,2,3,4
    for element in 1..5
    {
        print!("{} ",element);
    }
    print!("\n");

    // 5,4,3,2,1 note o =5 que inclui o extremo
    for element in (1..=5).rev()
    {
        print!("{} ",element);
    }
    print!("\n");

    let mut vet = [1,3,5];
    vet.reverse();
    // 5, 3, 1
    for element in vet
    {
        print!("{} ",element);
    }
    print!("\n");
}

fn expressoes()
{
    // expressoes retornam um valor por exemplo
    let ret = {
        let x= 10;
        x+15
        // esse escopo retorna o valor 10 + 15 logo ret = 25
    };
    println!("ret {}",ret);

    let x = 5;
    let ret = if x>0 {5} else {9};
    println!("ret {}",ret);

    let ret = if x>0
    {
        let a = 20;
        a
        // return a - erro o return nao é uma expressao
    }
    else
    {
        10
    };
    println!("ret {}",ret);
}

fn funcao()
{
    let ret = funcao_dev(2);
    println!("função retornou {ret}");
}
// tem que declarar tipo em paramtro
// -> o tipo do retorno
fn funcao_dev(param:u32) -> u32
{
    println!("param {}",param);
    if param == 0
    {

        return 1;
    }
    6 // uma expressão FINAL sem o ; no final é como se fosse return !!!!!!!!!!!!!!!!!!!
}
fn tipos()
{
    // contantes tem que ter tipo
    const A: u32 = 20;
    // const mut A: u32 = 20; => não faz sentido constante ser mutavel
    println!("{A}");
    //tipos
    let int32 :i32 = 20;
    let int8 :i8 = 2;
    
    // let int8 :u8 = 9999999999999 => erro overflow

    let sem_sinal_int32:u32 = 20;
    // let sem_sinal_int64:u32 = -10; => erro não pode ser sem sinal
    println!("{int32} {int8} {sem_sinal_int32}");

    // depende da arquitetura do pc (32 ou 64 bits)
    let depende_da_arquitetura:usize = 20;

    let deciomal = 1234;
    let hexa = 0x12E;
    let octa = 0o12;
    let binario = 0b11101;
    let byte = b'2';
    println!("{} {} {} {} {} {}", depende_da_arquitetura,deciomal, hexa, octa, binario, byte);

    let float32:f32 =1.06;
    let float64:f64 = 2.509;

    // let soma:f64 = float32 + float64; -> erro faz contas com tipos iguais só
    let soma:f64 = float64 + 1.50;
    println!("{float32} {float64} {soma} ");

    let booleano:bool = true;

    // let caracter:char= "a"; - erro tem que ser aspas simples igual o c
    let caracter:char = 'a';

    println!("{booleano} {caracter}");

    //tuplas
    let tupla: (i32, f64, u8) = (500, 6.4, 1);
    println!("{} {}", tupla.0, tupla.1);

    // let vetor:[i32;5] = [1,2,3,4]; - erro tem que ter 5 elementos
    let vetor:[i32;5] = [1,2,3,4,5];
    println!("{} {}", vetor[0], vetor[2]);
    let vetor_dinamico = [1,2,4,5];
    println!("{}", vetor_dinamico[2]);
    // vetor_dinamico[6] = 6 =>  erro se declarar com X valor nao da pra acrescentar
    // println!("{}", vetor_dinamico[6]);

    let vetor_repetido = [3;5]; //[3,3,3,3,3]
    println!("{}", vetor_repetido[2]);


}
fn sombreamento() {
    // declarar variaveis sem já falar o valor
    let a;
    a = 10;
    // a=20; => ERRO pois é imutavel
    println!("Valor de 'a' é {a}, 'a' é uma variavel IMUTAVEL");

    // redeclarar a variavel sobresceve o valor
    // isso é o sombreamento das variaveis
    let mut a;
    a = 20;
    println!("a variavel 'a' foi redeclarada agora ela é MUTAVEL e seu novo valor é {}", a);
    a = 30;
    println!("Como 'a' é mutavel eu consigo atribuir um novo valor que agora é {}", a);
    // as duas variaveis a estão no mesmoe escopo logo a ultima declaração é valida

    //testes de escopo
    println!("Estou no escopo 0");
    let x = 5;
    let x = x + 1; // x =6

    {
        println!("Estou no escopo 1");
        let x = x * 2;
        println!("O valor de 'x' dentro do escopo 1 é {}", x);
        println!("Saindo escopo 1");
    }
    println!("O valor de 'x' dentro do escopo 0 é {}", x);

    // sombreamento com mut
    println!("Estou no escopo 0");
    let mut x = 5;

    {
        println!("Estou no escopo 1");
        x =x+2; // mudo o valor do x de fora
        {
            println!("Estou no escopo 2.1");
            x = x-1; // muda o x de fora note que x = x + 2 -1 = 5 + 2 +1 = 6
            let x = -1;
            println!("O valor de 'x' dentro do escopo 2.1 é {}", x);
            println!("Saindo escopo 2.1");
        }
        let mut x = x * 2;
        println!("O valor de 'x' dentro do escopo 1 é {}", x);
        {
            println!("Estou no escopo 2.2");
            x = x-1; // note mudou o x do escopo mais acima em que ela foi declarada
            // ou seja x = x * 2 - 1 = 6 * 2 -1 = 11
            println!("O valor de 'x' dentro do escopo 2.2 {}", x);
            println!("Saindo escopo 2.2");
        }
        println!("O valor de 'x' dentro do escopo 1 apos o escopo 2.2 {}", x);
        println!("Saindo escopo 1");
    }

    println!("O valor de 'x' dentro do escopo 0 é {}", x);
    // quando usamos uma variavel dentro de outro escopo seu valor utilizado é do escopo pais EM QUE ELA FOI DECLARADA mais proximo
    // quando printamos usando x no escopo 2.1 o "let" do x mais proximo é do escopo 0 ou seja ela utiliza esse x
    // quando printamos usando x no escopo 2.2 o "let" de x mais proximo é no escopo 1 
    // x="a"; erro pois x é do tipo inteiro
}

fn teste_print() {
    println!("Hello, world!");

    // o ! significa macro do rust
    println!("Receber dado do usuario");

    //variavel mutavel, em rust todas são imutaveis (não muda)
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    // estrutura de ifs iguais
    let teste_string = "teste";
    if teste_string == "teste2" 
    {
        println!("teste_string é \"teste2\"");
    }
    else if teste_string == "teste"
    {
        println!("teste_string é \"teste\"");
    }
}
