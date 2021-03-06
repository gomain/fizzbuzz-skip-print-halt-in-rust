use fizzbuzz::{Condition, Program, Rule};

fn main() {
    let rules = vec![(3, "Fizz"), (5, "Buzz"), (7, "Hizz"), (11, "Howl")]
        .into_iter()
        .map(|(n, word)| Rule(Condition::HasAsFactor(n), word))
        .collect::<Vec<_>>();

    (1..1156) // 1155 is the first number that places all words!
        .for_each(|n| {
            let fizz_buzz = Program::new(&rules, n);
            //println!("{}", fizz_buzz.string());
            fizz_buzz.println();
        })
}
