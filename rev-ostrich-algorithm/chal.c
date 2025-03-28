#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <sys/types.h>
#include <openssl/md5.h>
#include <string.h>


char f[] = "oiiaoiiaoiiaoiia";
volatile int x;
int main(void) {
	char hey[] = "welcome to UTCTF!";
	for(int i = 0; i < 17; i++) {
		if(hey[i] != f[i])
			exit(0);
	}
		unsigned char digest[16];
		MD5_CTX ctx;
		MD5_Init(&ctx);
		MD5_Update(&ctx, (char *)&main, 32);
		MD5_Final(digest, &ctx);
		printf("utflag{");
		for(int i = 0; i < 16; i++)
			printf("%02x", (unsigned int)digest[i]);
		printf("}");
	return 0;
}
