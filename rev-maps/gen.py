#!/usr/bin/python3
import random

for i in range(200) :
    r1 = random.randint(0, 65536)
    print("f = real_transform<int, std::vector<int>, std::function<int(int)>>(f, ", end="")
    print("[] (int n) -> int {return n ^ " + str(r1) + "; });")
