pub trait lenguaje {
    fn sum(&self) -> String;
}

pub struct libro {
    pub headline: String,
    pub author: String,
    pub date: String,
    pub copy: int,
    pub id: String,
}


impl lenjuage for libro {
    fn sum(&self) -> String {
        format!("el nombre es {}, hecho por {} en la fecha {}")
    }
}