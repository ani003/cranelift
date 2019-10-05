.globl _uthread_ctx_switch
.text
_uthread_ctx_switch:
    movq %rsp, 16(%rdi)
    leaq (%rip), %rax
    movq %rax, 10(%rdi)
