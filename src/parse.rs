
#[derive(Debug, PartialEq)]
pub enum Instrs {
    Inc,
    Dec,
    Left,
    Right,
    Print,
    Read,
    Lp,
    LpEnd,
}

pub fn parser(s: String) -> Vec<Instrs> {
    let mut xs: Vec<Instrs> = Vec::new(); 
    for c in s.chars() {
        match c {
            '>' => xs.push(Instrs::Right),
            '<' => xs.push(Instrs::Left),
            '+' => xs.push(Instrs::Inc),
            '-' => xs.push(Instrs::Dec),
            '.' => xs.push(Instrs::Print),
            ',' => xs.push(Instrs::Read),
            '[' => xs.push(Instrs::Lp),
            ']' => xs.push(Instrs::LpEnd),
             _  => {} 
        }
    }
    xs
}