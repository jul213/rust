use std:fs:File;

enum result<T,E>{
    Ok(T),
    Err(E),
}

fn main(){
    let fichero = File::open("hola.txt");

    let file_succefuss = match fichero {
        Ok(file) => file,
        Err(error) => panic!("error en el sistema con el fichero"),
    };
}