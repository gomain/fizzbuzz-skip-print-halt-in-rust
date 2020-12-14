use std::fmt::Display;
use std::matches;
use std::ops;

pub struct Rule<'a>(pub Condition, pub &'a str);

pub struct Program<'a> {
    cmds: Vec<Cmd<'a>>,
}

enum Cmd<'a> {
    Skip,
    Halt,
    Print(Box<dyn Display + 'a>),
}

impl<'a> Program<'a> {
    pub fn new(rules: &'a [Rule<'a>], n: u32) -> Program<'a> {
        rules
            .into_iter()
            .filter_map(|Rule(cond, word)| if satisfy(cond, n) { Some(word) } else { None })
            .fold(
                ProgramWithHole::new(
                    Program::skip(),
                    Program::print(Box::new(n)) + Program::halt(),
                ),
                |acc, &word| {
                    acc + ProgramWithHole::new(Program::print(Box::new(word)), Program::halt())
                },
            )
            .into()
    }

    pub fn println(&self) {
        for cmd in self.cmds.iter() {
            match cmd {
                Cmd::Skip => continue,
                Cmd::Halt => {
                    print!("\n");
                    return;
                }
                Cmd::Print(it) => print!("{}", it),
            }
        }
    }

    pub fn string(&self) -> String {
        self.cmds
            .iter()
            .take_while(|cmd| !matches!(cmd, Cmd::Halt))
            .map(|cmd| {
                if let Cmd::Print(it) = cmd {
                    it.to_string()
                } else {
                    "".to_string()
                }
            })
            .collect()
    }

    fn cmd(cmd: Cmd<'a>) -> Program<'a> {
        Program { cmds: vec![cmd] }
    }

    fn halt() -> Program<'a> {
        Program::cmd(Cmd::Halt)
    }

    fn skip() -> Program<'a> {
        Program::cmd(Cmd::Skip)
    }

    fn print(it: Box<dyn Display + 'a>) -> Program<'a> {
        Program::cmd(Cmd::Print(it))
    }
}

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

struct ProgramWithHole<'a> {
    pre: Program<'a>,
    post: Program<'a>,
}

impl<'a> ProgramWithHole<'a> {
    fn new(pre: Program<'a>, post: Program<'a>) -> Self {
        ProgramWithHole { pre, post }
    }
}

impl<'a> From<ProgramWithHole<'a>> for Program<'a> {
    fn from(this: ProgramWithHole<'a>) -> Self {
        this.pre + this.post
    }
}

impl<'a> ops::Add for ProgramWithHole<'a> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        ProgramWithHole::new(self.pre + other.pre, other.post + self.post)
    }
}

pub enum Condition {
    HasAsFactor(u32),
}

pub fn satisfy(cond: &Condition, i: u32) -> bool {
    match cond {
        Condition::HasAsFactor(n) => i % n == 0,
    }
}
