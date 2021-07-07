.global start
.text
start:
    pushq %rbp
    movq %rsp, %rbp
    movq  $0, -8(%rbp)
    addq $1, -8(%rbp)
    addq $1, -8(%rbp)
    movq $0, -16(%rbp)
    addq $1, -16(%rbp)
    addq $1, -16(%rbp)
    addq $1, -16(%rbp)
    addq $1, -16(%rbp)
    addq $1, -16(%rbp)
    cmpq $0, -16(%rbp)
    je LOOP_END_0
    LOOP_START_0:
    addq $1, -8(%rbp)
    subq $1, -16(%rbp)
    cmpq $0, -16(%rbp)
    jne LOOP_START_0
    LOOP_END_0:
    addq $1, -16(%rbp)
    addq $1, -16(%rbp)
    addq $1, -16(%rbp)
    addq $1, -16(%rbp)
    addq $1, -16(%rbp)
    addq $1, -16(%rbp)
    addq $1, -16(%rbp)
    addq $1, -16(%rbp)
    cmpq $0, -16(%rbp)
    je LOOP_END_1
    LOOP_START_1:
    addq $1, -8(%rbp)
    addq $1, -8(%rbp)
    addq $1, -8(%rbp)
    addq $1, -8(%rbp)
    addq $1, -8(%rbp)
    addq $1, -8(%rbp)
    subq $1, -16(%rbp)
    cmpq $0, -16(%rbp)
    jne LOOP_START_1
    LOOP_END_1:
    movq  -8(%rbp), %rax
    popq %rbp
    ret
