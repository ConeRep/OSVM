%include "./libs/sys_libs.os"

_start: 
    mov r0, #17
    push #0
    push #1
loop:
    dupl 1
    dupl 1
    adds
    
    ; Print fibbonacci number using print_num system function
    mov r7, print_num
    sysf
    
    ; Decrement the counter
    dec r0
    jnz loop, r0
    hlt