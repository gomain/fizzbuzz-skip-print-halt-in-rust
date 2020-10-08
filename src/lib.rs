pub fn program<'a>(n: u32, rules: &'a Rules) -> Program<'a> {
    rules
        .into_iter()
        .filter(|(cond, _)| satisfy(cond, n))
        .fold(
            ProgramWithHole::new(Program::skip(), Program::print_num(n) + Program::halt()),
            |cur, (_, word)| {
                cur + ProgramWithHole::new(Program::print_string(word), Program::halt())
            },
        )
        .join()
}

pub type Rules<'a> = Vec<(Condition, &'a str)>;

#[derive(Clone, Debug)]
enum Cmd<'a> {
    Skip,
    Halt,
    PrintString(&'a str),
    PrintNum(u32),
}

#[derive(Clone, Debug)]
pub struct Program<'a> {
    cmds: Vec<Cmd<'a>>,
}

impl<'a> Program<'a> {
    pub fn run(&self) {
        for cmd in &self.cmds {
            match cmd {
                Cmd::Skip => continue,
                Cmd::Halt => {
                    print!("\n");
                    return;
                }
                Cmd::PrintString(it) => print!("{}", it),
                Cmd::PrintNum(n) => print!("{}", n),
            }
        }
    }

    fn single(cmd: Cmd<'a>) -> Program<'a> {
        Program { cmds: vec![cmd] }
    }

    fn halt() -> Program<'a> {
        Program::single(Cmd::Halt)
    }

    fn skip() -> Program<'a> {
        Program::single(Cmd::Skip)
    }

    fn print_string(s: &'a str) -> Program<'a> {
        Program::single(Cmd::PrintString(s))
    }

    fn print_num(n: u32) -> Program<'a> {
        Program::single(Cmd::PrintNum(n))
    }
}

use std::ops;

impl<'a> ops::Add for Program<'a> {
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

impl<'a> ops::Add for &'a Program<'a> {
    type Output = Program<'a>;
    fn add(self, other: &'a Program) -> Program<'a> {
        self.clone() + other.clone()
    }
}

impl<'a> ops::Add<&'a Program<'a>> for Program<'a> {
    type Output = Self;
    fn add(self, other: &'a Program) -> Self {
        self + other.clone()
    }
}

#[derive(Debug)]
struct ProgramWithHole<'a> {
    pre: Program<'a>,
    post: Program<'a>,
}

impl<'a> ProgramWithHole<'a> {
    fn new(pre: Program<'a>, post: Program<'a>) -> Self {
        ProgramWithHole { pre, post }
    }

    fn join(self) -> Program<'a> {
        self.pre + self.post
    }
}

impl<'a> ops::Add for ProgramWithHole<'a> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        ProgramWithHole::new(self.pre + other.pre, other.post + self.post)
    }
}

impl<'a> ops::Add for &'a ProgramWithHole<'a> {
    type Output = ProgramWithHole<'a>;
    fn add(self, other: &'a ProgramWithHole) -> ProgramWithHole<'a> {
        self.clone() + other.clone()
    }
}

impl<'a> ops::Add<&'a Program<'a>> for &'a ProgramWithHole<'a> {
    type Output = Program<'a>;
    fn add(self, other: &'a Program) -> Program<'a> {
        self.pre.clone() + other + self.post.clone()
    }
}

#[derive(Debug)]
pub enum Condition {
    MultipleOf(u32),
}

pub fn satisfy(cond: &Condition, i: u32) -> bool {
    match cond {
        Condition::MultipleOf(n) => i % n == 0,
    }
}
