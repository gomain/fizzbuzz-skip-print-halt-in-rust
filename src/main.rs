fn main() {
    let rules: Vec<(Condition, String)> = vec![(3, "Fizz"), (5, "Buzz"), (7, "Hizz"), (11, "Howl")]
        .into_iter()
        .map(|(n, word)| (Condition::MultipleOf(n), word.to_string()))
        .collect();
    let say_all = rules
        .iter()
        .fold("".to_string(), |cur, (_, word)| cur + word);
    for i in 1.. {
        let say = say(i, &rules);
        println!("{}", say);
        if say == say_all {
            break;
        }
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
        words.iter().fold("".to_string(), |cur, next| cur + next)
    } else {
        i.to_string()
    }
}

fn satisfy(cond: &Condition, i: u32) -> bool {
    match cond {
        Condition::MultipleOf(n) => i % n == 0,
    }
}
