
#[derive(Debug, PartialEq)]
pub enum Instrs {
    Inc,
    Dec,
    Left,
    Right,
    Print,
    Read,
    Lp(usize),
    LpEnd(usize),
}
pub fn parser(s: String) -> Vec<Instrs> {
    let mut xs: Vec<Instrs> = Vec::new(); 
    let mut lbl = 0;
    for  c in s.chars() {
        match c {
            '>' => xs.push(Instrs::Right),
            '<' => xs.push(Instrs::Left),
            '+' => xs.push(Instrs::Inc),
            '-' => xs.push(Instrs::Dec),
            '.' => xs.push(Instrs::Print),
            ',' => xs.push(Instrs::Read),
            '[' => xs.push(Instrs::Lp(lbl)),
            ']' => {xs.push(Instrs::LpEnd(lbl));
                    lbl += 1 //no nesting in the goonsquad compiler
            }

             _  => {} 
        }
    }
    xs
}


