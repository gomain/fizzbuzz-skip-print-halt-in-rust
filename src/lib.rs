use std::matches;
use std::ops;

pub struct Rule<'a>(pub Condition, pub &'a str);

#[derive(Clone, Debug)]
enum Cmd<'a> {
    Skip,
    Halt,
    PlaceString(&'a str),
    PlaceNum(u32),
}

#[derive(Clone, Debug)]
pub struct Program<'a> {
    cmds: Vec<Cmd<'a>>,
}

impl<'a> Program<'a> {
    pub fn new(rules: &'a [Rule<'a>], n: u32) -> Program<'a> {
        rules
            .into_iter()
            .filter_map(|Rule(cond, word)| if satisfy(cond, n) { Some(word) } else { None })
            .fold(
                ProgramWithHole::new(Program::skip(), Program::place_num(n) + Program::halt()),
                |acc, word| {
                    acc + ProgramWithHole::new(Program::place_string(word), Program::halt())
                },
            )
            .join()
    }

    pub fn println(&self) {
        for cmd in &self.cmds {
            match cmd {
                Cmd::Skip => continue,
                Cmd::Halt => {
                    print!("\n");
                    return;
                }
                Cmd::PlaceString(s) => print!("{}", s),
                Cmd::PlaceNum(n) => print!("{}", n),
            }
        }
    }

    pub fn string(&self) -> String {
        self.cmds
            .iter()
            .take_while(|cmd| !matches!(cmd, Cmd::Halt))
            .map(|cmd| match cmd {
                Cmd::PlaceString(s) => s.to_string(),
                Cmd::PlaceNum(n) => n.to_string(),
                _ => "".to_string(),
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

    fn place_string(s: &'a str) -> Program<'a> {
        Program::cmd(Cmd::PlaceString(s))
    }

    fn place_num(n: u32) -> Program<'a> {
        Program::cmd(Cmd::PlaceNum(n))
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
    HasAsFactor(u32),
}

pub fn satisfy(cond: &Condition, i: u32) -> bool {
    match cond {
        Condition::HasAsFactor(n) => i % n == 0,
    }
}
