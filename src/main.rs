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
}