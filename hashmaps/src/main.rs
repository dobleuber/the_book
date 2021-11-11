fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    println!("scores 1 {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("scores 2 {:?}", scores);


    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("score is {:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("scores 3 {:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Green")).or_insert(50);

    println!("scores 4 {:?}", scores);

}

