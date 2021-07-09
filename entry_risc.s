.data:
memory: .space 30000


.text
main:
    la s0, memory
    addi s0, s0, 1
    addi s0, s0, 1
    addi s0, s0, 1
    addi s0, s0, 1
    addi a0, x0, 1
    addi a1, s0, 0
    ecall
    lbu s1, (s0)
    addi s1, s1, 1
    sb s1, (s0)
    addi s0, s0, 1
    addi s0, s0, 1
    addi a0, x0, 1
    addi a1, s0, 0
    ecall
    addi 	a0, x0, 10
    ecall

