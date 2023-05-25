fn main() {
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
}