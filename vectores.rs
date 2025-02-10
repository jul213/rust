//cargo new vestores
// cd vestores
// cargo run

let v = vec![120,50,100,10,4,6];

for i in &v{
    println!("{i}");
}


let mut s = vec![10,20,40,5,90,1];

for e in mut &s{
    *e += 50;
}