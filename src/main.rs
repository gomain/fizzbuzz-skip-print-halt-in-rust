fn main() {
    let rules = vec![
        (Condition::MultipleOf(3), "fizz".to_string()),
        (Condition::MultipleOf(5), "buzz".to_string()),
        (Condition::MultipleOf(7), "hurray".to_string()),
    ];
    for i in 1..106 {
        println!("{}", say(i, &rules));
    }
}

enum Condition {
    MultipleOf(u32),
}

fn say(i: u32, rules: &Vec<(Condition, String)>) -> String {
    let words: Vec<&str> = rules
        .into_iter()
        .filter(|(cond, _)| satisfy(cond, i))
        .map(|(_, word)| -> &str { word })
        .collect();
    if words.len() > 0 {
        words.join("")
    } else {
        i.to_string()
    }
}

fn satisfy(cond: &Condition, i: u32) -> bool {
    match cond {
        Condition::MultipleOf(n) => i % n == 0,
    }
}
