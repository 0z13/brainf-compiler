#![allow(unused)]
use crate::parse::Instrs;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;

pub fn init_file () ->  std::io::Result<File> {
    let path = Path::new("entry.s");
    let mut f = std::fs::File::create(path)?;
    f.write_all(b".section .text\n    .global entry\nentry:\n    pushq %rbp\n    movq %rsp, %rbp\n");
    Ok(f)
}


pub fn compile(inp: Vec<Instrs>) -> std::io::Result<()> {
    let mut ptr:i32 = 0;
    let mut f = init_file().expect("io err");
    
    for i in inp {
        match i {
            Instrs::Inc => {
                let instrs = format!("    addq $1, {}(%rsp)\n", ptr);
                f.write_all(instrs.as_bytes());
            } 
            Instrs::Dec => {
                let instrs = format!("    subq $1, {}(%rsp)\n", ptr);
                f.write_all(instrs.as_bytes());
            } 
            Instrs::Right => {
                ptr -= 8;
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
