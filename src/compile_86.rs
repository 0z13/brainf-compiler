#![allow(unused)]
use crate::parse::Instrs;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;

pub fn init_file () ->  std::io::Result<File> {
    let path = Path::new("entry.s");
    let mut f = std::fs::File::create(path)?;
    f.write_all(b".bss\nlcomm ARRAY 30000\n.text\n.global start\nstart:\n");
    Ok(f)
}


pub fn compile(inp: Vec<Instrs>) -> std::io::Result<()> {
    let mut f = init_file().expect("io err");
    let mut instr_list: Vec<String> = Vec::new(); 
    for i in inp {
        match i {
            Instrs::Inc => {
                let instr = format!("    add $1, %r12\n");
                instr_list.push(instr);
            } 
            Instrs::Dec => {
                let instr = format!("    subq $1, %rsp\n");
                instr_list.push(instr);
            } 
            Instrs::Right => {
                
            }
            Instrs::Left => {
                ptr += 8;
                if ptr > 0 {
                    panic!("Buffer overflow!");
                }
            }
            Instrs::Read => {
                let mut s = String::from(""); // User input
                std::io::stdin().read_line(&mut s).expect("input is invalid");
                let tmp = format!("    movq ${} {}($rsp)", s, ptr);
                f.write_all(tmp.as_bytes());
            }
            Instrs::Print => {
                // How do i print in assembly?
            }
            Instrs::Lp => {
                // I have to do some branching
            }
            Instrs::LpEnd => {
                // I have to do some branching
            }
        }
    }
    // todo
    // install nasm 
    // call c entry
    // try to mess with cargo pipeline

    f.write_all(b"    retq");
    Ok(())
}
