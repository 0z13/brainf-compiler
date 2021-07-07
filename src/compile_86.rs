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
    let mut curr_index:usize = 0;
    let mut arr: [bool; 100] = [false; 100];
    arr[0] = true;

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
                curr_index += 1;
                if (!arr[curr_index]) {
                    let instr = format!("    movq $0, {}(%rbp)\n", curr_pos);
                    instr_list.push(instr);
                    arr[curr_index] = true
                } else {
                    // no zero-init
                }
            }
            Instrs::Left => {
                curr_pos = curr_pos + 8;
                curr_index -= 1;
                if (!arr[curr_index]) {
                    let instr = format!("    movq $0, {}(%rbp)\n", curr_pos);
                    instr_list.push(instr);
                    arr[curr_index] = true
                } else {
                   // no need to 0 initialize     
                }
            }
            Instrs::Read => {
                // sigh
            }
            Instrs::Print => {
                // sigh
            }
            Instrs::Lp(index) => {
                let instr = 
                format!("    cmpq $0, {}(%rbp)\n    je LOOP_END_{}\n    LOOP_START_{}:\n", curr_pos, index, index);
                instr_list.push(instr);
            }
            Instrs::LpEnd(index) => {
                let instr = 
                format!("    cmpq $0, {}(%rbp)\n    jne LOOP_START_{}\n    LOOP_END_{}:\n", curr_pos, index, index);
                instr_list.push(instr);
            }
        }
    }
                
    let l1 = format!("    movq  {}(%rbp), %rax\n    popq %rbp\n", curr_pos);
    let l2 = format!("    ret\n");
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
