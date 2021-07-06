#![allow(unused)]
use crate::parse::Instrs;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;

pub fn init_file () ->  std::io::Result<File> {
    let path = Path::new("entry.s");
    let mut f = std::fs::File::create(path)?;
    f.write_all(b".bss\n.lcomm ARRAY, 30000\n.text\n.global start\nstart:\n");
    Ok(f)
}


pub fn compile(inp: Vec<Instrs>) -> std::io::Result<()> {
    let mut f = init_file().expect("io err");
    let mut instr_list: Vec<String> = Vec::new(); 
    for i in inp {
        match i {
            Instrs::Inc => {
                let instr = format!("    addb $1, (%r12)\n");
                instr_list.push(instr);
            } 
            Instrs::Dec => {
                let instr = format!("    subb $1, (%r12)\n");
                instr_list.push(instr);
            } 
            Instrs::Right => {
                let instr = format!("    addb (%r12) $1, 1\n");
                instr_list.push(instr);
            }
            Instrs::Left => {
                let instr = format!("    addb (%r12) $1, 1\n");
                instr_list.push(instr);
            }
            Instrs::Read => {
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
    let l1 = format!("    movb  (%r12), %rax\n");
    let l2 = format!("    retq");
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
