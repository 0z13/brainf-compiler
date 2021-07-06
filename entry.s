.bss
.lcomm ARRAY, 30000
.text
.global start
start:
    mov %r12, offset ARRAY
    addb $1, (%r12)
    addb $1, (%r12)
    subb $1, (%r12)
    addb $1, (%r12)
    movb  %r12, %al
    retq
