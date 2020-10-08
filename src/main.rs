fn main() {
    let rules: Rules = vec![(3, "Fizz"), (5, "Buzz"), (7, "Hizz"), (11, "Howl")]
        .iter()
        .map(|(n, word)| (Condition::MultipleOf(*n), word.to_string()))
        .collect();

    for n in 1.. {
        print!("{}:\t", n);
        program(n, &rules).run();
        if rules.iter().all(|(cond, _)| satisfy(cond, n)) {
            break;
        }
    }
}

fn program(n: u32, rules: &Rules) -> Program {
    rules
        .into_iter()
        .filter(|(cond, _)| satisfy(cond, n))
        .fold(
            ProgramWithHole::new(Program::skip(), Program::print_num(n) + Program::halt()),
            |cur, (_, word)| {
                cur + ProgramWithHole::new(Program::print_string(word.clone()), Program::halt())
            },
        )
        .join()
}

type Rules = Vec<(Condition, String)>;

#[derive(Clone, Debug)]
enum Cmd {
    Skip,
    Halt,
    PrintString(String),
    PrintNum(u32),
}

#[derive(Clone, Debug)]
struct Program {
    cmds: Vec<Cmd>,
}

impl Program {
    fn run(&self) {
        for cmd in &self.cmds {
            match cmd {
                Cmd::Skip => continue,
                Cmd::Halt => {
                    println!("");
                    return;
                }
                Cmd::PrintString(it) => print!("{}", it),
                Cmd::PrintNum(n) => print!("{}", n),
            }
        }
    }

    fn single(cmd: Cmd) -> Program {
        Program { cmds: vec![cmd] }
    }

    fn halt() -> Program {
        Program::single(Cmd::Halt)
    }

    fn skip() -> Program {
        Program::single(Cmd::Skip)
    }

    fn print_string(s: String) -> Program {
        Program::single(Cmd::PrintString(s))
    }

    fn print_num(n: u32) -> Program {
        Program::single(Cmd::PrintNum(n))
    }
}

use std::ops;

impl<'a> ops::Add for Program {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Program {
            cmds: self
                .cmds
                .into_iter()
                .chain(other.cmds.into_iter())
                .collect(),
        }
    }
}

impl ops::Add for &Program {
    type Output = Program;
    fn add(self, other: &Program) -> Program {
        self.clone() + other.clone()
    }
}

impl<'a> ops::Add<&Program> for Program {
    type Output = Self;
    fn add(self, other: &Program) -> Program {
        self + other.clone()
    }
}

#[derive(Debug)]
struct ProgramWithHole {
    pre: Program,
    post: Program,
}

impl ProgramWithHole {
    fn new(pre: Program, post: Program) -> Self {
        ProgramWithHole { pre, post }
    }

    fn join(self) -> Program {
        self.pre + self.post
    }
}

impl ops::Add for ProgramWithHole {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        ProgramWithHole::new(self.pre + other.pre, other.post + self.post)
    }
}

impl ops::Add for &ProgramWithHole {
    type Output = ProgramWithHole;
    fn add(self, other: &ProgramWithHole) -> ProgramWithHole {
        self.clone() + other.clone()
    }
}

impl ops::Add<&Program> for &ProgramWithHole {
    type Output = Program;
    fn add(self, other: &Program) -> Program {
        self.pre.clone() + other + self.post.clone()
    }
}

#[derive(Debug)]
enum Condition {
    MultipleOf(u32),
}

fn satisfy(cond: &Condition, i: u32) -> bool {
    match cond {
        Condition::MultipleOf(n) => i % n == 0,
    }
}
