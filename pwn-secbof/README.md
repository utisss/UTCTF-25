Basic buffer overflow w/ seccomp filter.

Binary is statically compiled (so there are syscall gadgets,
along with gadgets to load rdx/rdi/rsi). No PIE enabled either,
so it's just a simple ROP chain to call open -> read -> write.

