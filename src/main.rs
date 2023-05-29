static GLOBAL_VAR: u8 = 1;
static mut MUTABLE_GLOBAL_VAR: u8 = 2;

fn soma(a:i32, b:i32) -> i32 {
    let resultado = a + b;
    println!("Resultado: {resultado}");
    return resultado;
}

fn shadow() {
    {
        let dentro = "teste";
        println!("dentro {dentro}");
    }

    //println!("fora {dentro}");
}

fn index() {
    println!("Hello world");
    let mut variable = "mutable_variable";
    println!("First: {variable}");
    variable = "variable";
    println!("Second: {variable}");
    let sec_variable = "immutable_variable";
    println!("First: {sec_variable}");
    const VARIABLE_CONSTANTE: &str = "variavel constante";
    println!("{VARIABLE_CONSTANTE}");
    let numero: u8 = 3;
    let sec_numero = 2023;
    println!("variavel: {}, tamanho = {} bytes", numero, std::mem::size_of_val(&numero));
    println!("variavel: {}, tamanho = {} bytes", sec_numero, std::mem::size_of_val(&sec_numero));

    let floaters: f32 = 2.5;
    println!("variavel: {}, tamanho = {} bytes", floaters, std::mem::size_of_val(&floaters));

    let verdadeiro: bool = true;
    println!("{verdadeiro}");

    println!("Global variable: {GLOBAL_VAR}");

    unsafe {
        println!("Unsafe block of code: {MUTABLE_GLOBAL_VAR}");
    };
}

fn main() {
    soma(1, 2);
    shadow();
    index();
}