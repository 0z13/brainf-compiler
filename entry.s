.global start
.text
start:
    pushq %rbp
    movq %rsp, %rbp
    movq  $0, -8(%rbp)
    addq $1, -8(%rbp)
    addq $1, -8(%rbp)
    subq $1, -8(%rbp)
    addq $1, -8(%rbp)
    movq  -8(%rbp), %rax
    popq %rbp
    ret