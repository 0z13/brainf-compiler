mod compile_86;
mod compile_riscv;
mod parse;

use compile_86::compile;
use parse::parser;
use parse::Instrs;
use std::io;
#[allow(unused)]



fn interp(xs: Vec<Instrs>) -> u8 {
    let mut mem = [0 as u8; 30000];
    let mut ptr: usize = 0;
    let mut iptr: usize = 0;
    // loop start and end markers
    let mut lps:usize = 0;
    let mut lpe: usize = 0;
    while iptr < xs.len() {
       match xs[iptr] {
           Instrs::Right => {
               ptr += 1;
               assert!(ptr <= 30000);}
           Instrs::Left  => {
               ptr -= 1;
               assert!(ptr >= 0)},
            Instrs::Dec => { mem[ptr] -= 1 },
            Instrs::Inc => { mem[ptr] += 1 },
            Instrs::Print => {
                if mem[ptr] < 127 {
                    println!("{}", mem[ptr] as char );
                } else {
                    println!("{:?}", mem[ptr]);
                }
            }
            Instrs::Read  => {
                let mut s = String::from("");
                io::stdin().read_line(&mut s).expect("Not integer input");
                let v:u8= s.parse().expect("Not integer input.");
                mem[ptr] = v;
            }
            Instrs::Lp(x) => {
                lps = iptr;
                if mem[ptr] == 0 {
                    assert!(xs[0] != Instrs::LpEnd(x));
                    iptr = lpe;
                }
            }
            Instrs::LpEnd(_) => {
                lpe = iptr;
                if mem[ptr] != 0 {
                    iptr = lps;
                } 
            }
       } 
       iptr += 1;
    } 
    mem[ptr]
}


fn main() -> () {
    // let stex1 = include_str!("./examples/helloworld.bf");
    // let ex1 = stex1.to_string();
    // let vex1 = parser(ex1);
    // let _whatever = init_file().unwrap();
    // let _x = interp(vex1);

        let xs = parser(include_str!("./examples/seven.bf").to_string());
        for i in &xs {
            println!("{:?}", i)
        }
        let _a = compile(xs);
        


        let testcase= parser(String::from("++++.>++."));
        compile_riscv::compile(testcase);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse1() {
        let xs = parser(String::from(">"));
        assert_eq!(xs[0], Instrs::Right)
    }
    #[test]
    fn test_parse2() {
        let xs = parser(String::from(">     \n \t <"));
        assert_eq!(xs[1], Instrs::Left)
    }
    #[test]
    fn test_interp1() {
        let xs = parser(String::from("++"));
        assert_eq!(2, interp(xs));
    }

    #[test]
    fn test_interp2() {
        let xs = parser(String::from("++>+++"));
        assert_eq!(3, interp(xs));
    }

}

