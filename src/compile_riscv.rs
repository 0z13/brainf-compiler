

// Compile things 
use crate::parse::Instrs;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;

fn init_file () ->  std::io::Result<File> {
    let path = Path::new("entry_risc.s");
    let mut f = std::fs::File::create(path)?;
    f.write_all(b".data:\nmemory: .space 30000\n\n\n.text\nmain:\n    la s0, memory\n")?;
    Ok(f)
}
// RISC-V Brainfuck compiler 
// written for the venus intrepeter
// available at https://venus.cs61c.org/


pub fn compile(instrs: Vec<Instrs>) -> std::io::Result<()> {
    let mut f:File = init_file()?;
    let mut stack_size = 0;
    let mut instr_list: Vec<String> = Vec::new(); 
    for i in instrs {
        match i {
            Instrs::Inc => {
                let ins = "    addi s0, s0, 1\n".to_string();
                instr_list.push(ins)
            }
            Instrs::Dec => {
                let ins = "    addi s0, s0, -1\n".to_string();
                instr_list.push(ins)
            }
            Instrs::Right => {
                let ins = format!("    lbu s1, (s0)\n    addi s1, s1, 1\n    sb s1, (s0)\n");
                instr_list.push(ins)
            }
            Instrs::Left => {
                let ins = format!("    lbu s1, (s0)\n    addi s1, s1, -1\n    sb s1, (s0)\n");
                instr_list.push(ins)
            }
            Instrs::Print => {
                let ins = format!("    addi a0, x0, 1\n    addi a1, s0, 0\n    ecall\n");
                instr_list.push(ins);
            }
            Instrs::Read => {
                // eh
            }
            Instrs::Lp(_c) => {
                // loop start
            }
            Instrs::LpEnd(_c) => {
                // lp end
            }
            
        }
    }

    // ugly but whatever
	let ins1 = format!("    addi 	a0, x0, 10\n");
	let ins2 = format!("    ecall\n\n");
    instr_list.push(ins1);
    instr_list.push(ins2);

    for i in &instr_list {
        f.write_all(i.as_bytes())?
    }
    Ok(())
}