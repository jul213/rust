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