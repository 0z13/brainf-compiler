# ./examples/x86_64/switch_case.s

# GNU Assembler, Intel syntax, x86_64 Linux

.data

# exit(code)
.equ SYS_EXIT, 60
.equ EXIT_CODE, 0

# write(fd, buf_adr, buf_len)
.equ SYS_WRITE, 1
.equ STDOUT, 1

# read(fd, buf_adr, buf_len)
.equ SYS_READ, 0
.equ STDIN, 0

# ASCII code for lowercase 'a'
.equ ASCII_A, 97

# Quick ASCII refresher:
# 65 - 91 = 'A' - 'Z'
# 97 - 123 = 'a' - 'z'

# e.g.
# 'A' + 32 = 'a'
# 'a' - 32 = 'A' 
.equ CASE_DIFF, 32

# single byte in memory
CHAR:
    .byte 0

.text

.global start

start:
    # read(STDIN, CHAR, 1)
    mov rax, SYS_READ
    mov rdi, STDIN
    mov rsi, offset CHAR
    mov rdx, 1
    syscall

    cmpb [CHAR], ASCII_A        # if byte at CHAR is lowercase
    jge MAKE_UPPERCASE          # make it uppercase

MAKE_LOWERCASE:                 # else make it lowercase
    addb [CHAR], CASE_DIFF      # lowercase byte at CHAR
    jmp WRITE                   # then write it to stdout

MAKE_UPPERCASE:
    subb [CHAR], CASE_DIFF      # uppercase byte at CHAR

WRITE:                          # write byte to stdout
    # write(STDOUT, CHAR, 1)
    mov rax, SYS_WRITE
    mov rdi, STDOUT
    mov rsi, offset CHAR
    mov rdx, 1
    syscall

    # exit(EXIT_CODE)
    mov rax, SYS_EXIT
    mov rdi, EXIT_CODE
    syscall
