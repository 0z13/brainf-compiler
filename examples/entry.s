.global start
.text
start:
    # read(STDIN, CHAR, 1)
    movq $5, %rax 
    movq $10, %r12
    cmp %rax, %r12
    jge RAX_IS_LARGER

    r12_is_larger:
    movq $100, %rax
    jmp END

    RAX_IS_LARGER:
    movq $200, %rax


    END: 
    retq
