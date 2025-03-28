#include <stdio.h>
#include <stddef.h>
//copied from man page
       #include <linux/seccomp.h>  /* Definition of SECCOMP_* constants */
       #include <linux/filter.h>   /* Definition of struct sock_fprog */
       #include <linux/audit.h>    /* Definition of AUDIT_* constants */
       #include <sys/syscall.h>    /* Definition of SYS_* constants */
		#include <sys/prctl.h>
		#include <stdlib.h>
       #include <unistd.h>
#include <fcntl.h>
#define ALLOW_SYSCALL(syscall) \
	BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, __NR_##syscall, 0, 1), \
	BPF_STMT(BPF_RET | BPF_K, SECCOMP_RET_ALLOW),
void install_filter(void) {
	struct sock_filter filter[] = {
		BPF_STMT(BPF_LD | BPF_W | BPF_ABS, offsetof(struct seccomp_data, arch)),
		BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, AUDIT_ARCH_X86_64, 1, 0),
		BPF_STMT(BPF_RET | BPF_W, SECCOMP_RET_KILL),
		BPF_STMT(BPF_LD | BPF_W | BPF_ABS, offsetof(struct seccomp_data, nr)),
		ALLOW_SYSCALL(read)
		ALLOW_SYSCALL(write)
		ALLOW_SYSCALL(open)
		ALLOW_SYSCALL(exit)
		BPF_STMT(BPF_RET | BPF_K, SECCOMP_RET_KILL)
	};
	if(prctl(PR_SET_NO_NEW_PRIVS,1,0,0,0)) {
		perror("set no new privs failed ");
		exit(-1);
	}
	struct sock_fprog prog = {
		.len = (unsigned short)(sizeof(filter) / sizeof(struct sock_filter)),
		.filter = filter
	};
	if(prctl(PR_SET_SECCOMP,SECCOMP_MODE_FILTER, &prog) == -1) {
		perror("setting filter failed ");
		exit(-1);
	}
}

int main(void) {
	setvbuf(stdout, NULL, _IONBF, 0);
	setvbuf(stdin, NULL, _IONBF, 0);
	install_filter();
	//vulnerable code goes here.
	char hi[128];
	printf("Input> ");
	read(0, hi, 1000);
	printf("Flag: ");
	return 0;
}
