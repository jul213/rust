// cargo new entrada
// cd entrada
// cargo run


use std::io;  //utilizar io


fn main() {

    println!("bienvenido al sistema de entrada en rust");

    println!("ingresa una entrada usuario: ");

    let mut salida = String::new();

    io::stdin()
    .read_Line(&mut salida)
    .expect("hubo un fallo en la lectura");

    println!("has escrito {}", salida);


}
