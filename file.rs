use std:fs:File;

enum result<T,E>{
    Ok(T),
    Err(E),
}

fn main(){
    let fichero = File::open("hola.txt");
}