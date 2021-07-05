.bss
.lcomm ARRAY, 30000
.text
.global start

start:
  movq $5, %rax
  retq 

