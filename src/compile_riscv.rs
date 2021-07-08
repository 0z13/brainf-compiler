

// Compile things 
use crate::parse::Instrs;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;

fn init_file () ->  std::io::Result<File> {
    let path = Path::new("entry_risc.s");
    let mut f = std::fs::File::create(path)?;
    f.write_all(b".global start\n.text\nstart:\n");
    Ok(f)
}


pub fn compile_riscv(instrs: Vec<Instrs>) -> std::io::Result {
    let mut stack_size = 0;
    let instr_list: Vec<String> = Vec![];
    for i in instrs {
        match i {
            Instr::Inc => {
                let ins = format!("lw x9 {}(sp)\n   addi x9 x9 1\n      sw {}(sp) x9\n", stack_size, stack_size); // temporary reg x9, maybe that's wrong? calle/caller thingi
                instr_list.push(ins)
            }
            Instr::Dec => {
                let ins = format!("lw x9 {}(sp)\n   addi x9 x9 -1\n      sw {}(sp) x9\n", stack_size, stack_size); // temporary reg x9, maybe that's wrong? calle/caller thingi
                instr_list.push(ins)
            }
            Instr::Right => {
                let ins = format!("    addi sp, sp, -8\n    ");
                stack_size -= 4
                // Maybe i just add 8 bytes to the stack every time, but
                // allocate only 4?
                // how do i keep the stack quadword aligned?/
            }
        }
    }


    
    Ok(())
}