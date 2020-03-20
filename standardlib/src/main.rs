use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    v.push(1);

    for i in &v2 {
        println!("{}", i)
    }

    let mut scores_map = HashMap::new();

    scores_map.insert(String::from("Blue"), 10);
    scores_map.insert(String::from("Red"), 5);

    let teams = vec![String::from("Blue"), String::from("Red"), String::from("Yellow")];

    let new_v = 15;
    for team_name in &teams {
        let new_entry = scores_map.entry(team_name.to_string()).or_insert(new_v);
        *new_entry += 1;
    }
    for (k, v) in scores_map {
        println!("{} - {}", k, v);
    }

}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}