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



fn largest(list: &i32) -> &i32 {
    let mut largest = &list[0];

    for item in list{
        if item > largest{
            largest = item;
        }
    }
    largest
}



fn largest_char(list: &[char]) -> &char{
    let mut largest = &list[0];

    for item in list{

        if item > largest{
            largest = item;
        } 
    }
    largest
} 


fn largestAll<T>(list: &[T])-> &T{
    let mut largest = &T[0];

    for item in list{
        if item > largest{
            largest = item;
        }
    }
    largest
}