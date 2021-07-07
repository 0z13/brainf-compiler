#![allow(unused)]
use crate::parse::Instrs;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;

pub fn init_file () ->  std::io::Result<File> {
    let path = Path::new("entry.s");
    let mut f = std::fs::File::create(path)?;
    f.write_all(b".global start\n.text\nstart:\n    pushq %rbp\n    movq %rsp, %rbp\n    movq  $0, -8(%rbp)\n");
    Ok(f)
}


pub fn compile(inp: Vec<Instrs>) -> std::io::Result<()> {
    let mut f = init_file().expect("io err");
    let mut instr_list: Vec<String> = Vec::new(); 
    let mut curr_pos:i32 = -8;
    for i in inp {
        match i {
            Instrs::Inc => {
                let instr = format!("    addq $1, {}(%rbp)\n", curr_pos);
                instr_list.push(instr);
            } 
            Instrs::Dec => {
                let instr = format!("    subq $1, {}(%rbp)\n", curr_pos);
                instr_list.push(instr);
            } 
            Instrs::Right => {
                curr_pos = curr_pos - 8;
                let instr = format!("    movq $0 {}(%rbp)\n", curr_pos);
                instr_list.push(instr);
            }
            Instrs::Left => {
                curr_pos = curr_pos + 8;
                let instr = format!("    movq $0 {}(%rbp)\n", curr_pos);
                instr_list.push(instr);
            }
            Instrs::Read => {
            }
            Instrs::Print => {
            }
            Instrs::Lp => {
                // I have to do some branching
            }
            Instrs::LpEnd => {
                // I have to do some branching
            }
        }
    }
                
    let l1 = format!("    movq  {}(%rbp), %rax\n    popq %rbp\n", curr_pos);
    let l2 = format!("    ret");
    instr_list.push(l1);
    instr_list.push(l2);
    for i in &instr_list {
        f.write_all(i.as_bytes());
    }
 

    // todo
    // install nasm 
    // call c entry
    // try to mess with cargo pipeline

    Ok(())
}
