use fizzbuzz::{program, Condition};

fn main() {
    let rules = vec![(3, "Fizz"), (5, "Buzz"), (7, "Hizz"), (11, "Howl")]
        .into_iter()
        .map(|(n, word)| (Condition::MultipleOf(n), word))
        .collect();

    (1..1156) // 1155 is the first number that prints all words!
        .for_each(|n| {
            print!("{}:\t", n);
            program(n, &rules).run();
        })
}
