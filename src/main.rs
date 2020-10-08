use fizzbuzz::{program, satisfy, Condition };

fn main() {
    let rules = vec![(3, "Fizz"), (5, "Buzz"), (7, "Hizz"), (11, "Howl")]
        .into_iter()
        .map(|(n, word)| (Condition::MultipleOf(n), word))
        .collect();

    for n in 1.. {
        print!("{}:\t", n);
        program(n, &rules).run();
        if rules.iter().all(|(cond, _)| satisfy(cond, n)) {
            break;
        }
    }
}
