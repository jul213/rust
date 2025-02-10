use std::collections:HashMap;

let mut scores = HashMap.new();

scores.insert(String::from("buenas tardes"), 17);
scores.insert(String::from("buenas noches"), 20);


for (key,values) in &scores{
    println!("{key}-{scores}");
}