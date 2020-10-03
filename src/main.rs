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
    let mut words = vec![i.to_string()];
    for (cond, word) in rules {
        if satisfy(cond, i) {
            words.push(word.to_string());
        }
    }
    join_tail_or_head(&words)
}

fn satisfy(cond: &Condition, i: u32) -> bool {
    match cond {
        Condition::MultipleOf(n) => i % n == 0,
    }
}

fn join_tail_or_head(words: &Vec<String>) -> String {
    if words.len() == 1 {
        words[0].to_string()
    } else {
        words[1..].join("_").to_string()
    }
}
