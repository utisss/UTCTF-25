#!/usr/bin/python3
from pwn import *
import random

"""
Design
you must navigate a maze designed as a graph --- start at a certain node, end at another node
Each node becomes an index in an arr
each arr[i] contains another array
bad choices (trying to run into a wall) are invalid instruction sequences in arr[i]
good choices (going to another node) load a value in rax/ret that will be used as an index in arr[i]
each arr[i] -> code[256]
where each code[x] will either be invalid instructions, instructions that store an index of another
node in the ret value, or they return nonsense in the ret value
we will minimum need 44 nodes
arr can have max 128 elements
"""


header = """
#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <string.h>
void * instr_buf;
int exec(void) {
	return ((int (*)(void))instr_buf)();
}

int execute(long instr) {
	memcpy(instr_buf, &instr, sizeof(long));
	return exec();
}

"""

flag = "utflag{1_w4nna_pl4y_hypix3l_in_c}"

"""
generate statements to initialize/overwrite arrays depending on flag
one function for each character
we will manually set each offset
all offsets but the correct one will be set to a randomly-generated value
correct offset will be set to either ret or pop <register>; jmp <register>
"""
def GCfun_for_char(buf_name, outgoing_edges, my_idx, mapping) :
    #code = f"int {fun_name}(char f) \u007b \n"
    #code += f"\tlong {buf_name}[128];\n"
    code = ""
    for i in range(128) :
        if not i in outgoing_edges.keys() :
            filler = random.randint(0, 0xffffffff)
            code += f"\t{buf_name}[{i + my_idx * 256}] = {filler};"
        else :
            thingy = 0xc358006a | (mapping[outgoing_edges[i]] << 8)
            print("Thingy: " + hex(thingy))
            code += f"\t//intended:\n"
            code += f"\t{buf_name}[{i + my_idx * 256}] = {hex(thingy)};"
        code += "\n"
    #code += f"\treturn execute({buf_name}[f]);\n"
    #code += f"\u007d \n"
    return code
"""
Generate the entire graph, given nodes + edges
nodes are numbers, edges is a dictionary of letter --> node number
"""
def generate_fun_graph(g, buf_name) :
    #there are 128 indexes, we generate a mapping
    print("")
    node_to_idx = { }
    taken = []
    for n in g.keys() :
        while True :
            r = random.randint(1, 127)
            if r not in taken :
                node_to_idx[n] = r
                taken.append(r)
                break
    ret = ""
    for n in g.keys() :
        ret = ret + GCfun_for_char(buf_name, g[n], node_to_idx[n], node_to_idx)
    return (node_to_idx, ret)

g = {1 : {ord('u') : 2}, 
     2 : {ord('t') : 3},
     3 : {ord('f') : 4},
     4 : {ord('l') : 5},
     5 : {ord('a') : 6},
     6 : {ord('g') : 7},
     7 : {ord('{') : 8, ord('f') : 20},
     8 : {ord('1') : 9},
     9 : {ord('_') : 10},
     10 : {ord('w') : 11, ord('i') : 1},
     11 : {ord('4') : 12},
     12 : {ord('n') : 13},
     13 : {ord('n') : 14},
     14 : {ord('a') : 15},
     15 : {ord('_') : 16},
     16 : {ord('p') : 17},
     17 : {ord('l') : 18},
     18 : {ord('4') : 19},
     19 : {ord('y') : 20, ord('u') : 11},
     20 : {ord('_') : 21},
     21 : {ord('h') : 22},
     22 : {ord('y') : 23},
     23 : {ord('p') : 24},
     24 : {ord('i') : 25},
     25 : {ord('x') : 26, ord('m') : 23},
     26 : {ord('3') : 27},
     27 : {ord('l') : 28},
     28 : {ord('_') : 29, ord('a') : 26},
     29 : {ord('i') : 30},
     30 : {ord('n') : 31},
     31 : {ord('_') : 32},
     32 : {ord('c') : 33},
     33 : {ord('}') : 1},
    }
(mapping, r) = generate_fun_graph(g, "meow")

main_code = """

int main(void) {
	instr_buf = mmap(NULL, 0x1000, PROT_EXEC | PROT_WRITE | PROT_READ, MAP_PRIVATE | MAP_ANONYMOUS,
		-1, 0);
	if(instr_buf == -1) {
		perror("Error: couldn't mmap instruction buffer ");
		exit(-1);
	}
	char * input = malloc(35);
	printf("Flag> ");
	fgets(input, 34, stdin);
    long * meow = malloc(sizeof(long) * 32768);
"""
main_code += f"int cur = {mapping[1]};"

all_code = header

all_code += main_code
all_code += r
all_code += f"meow[{256 * mapping[32]}] = 0xc358006a;"
all_code += """

    for(int i = 0; i < 33; i++) {
        cur = execute(meow[256 * cur + input[i]]);
    }
"""
all_code += "}"

f = open("./gen.c", "a")
f.write(all_code)
