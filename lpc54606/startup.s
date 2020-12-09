.text
.globl __pre_init
__pre_init:
        ldr r0, =0x40000220
        mov r1, #56
        str r1, [r0]
        bx lr
