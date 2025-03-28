import os
import random
import string

flag = 'utflag{3xp3rt_f0r3ns1c_4n4lys1s_:3c}'

number = int.from_bytes(flag.encode('utf-8'), byteorder='big')
octstring = oct(number)[2:]
print(octstring)
print(len(octstring))
length = 1024
filenameLength = 16

for i in range(len(octstring) // 3):
    sub = octstring[(i*3):((i+1)*3)]
    randomString = ''.join(random.choices(string.ascii_letters + string.digits, k=length))
    randomName = ''.join(random.choices(string.ascii_letters + string.digits, k=filenameLength))
    os.system(f'echo {randomString} >> {randomName}.txt')
    os.system(f'touch -d "2025-03-11 20:{23+i}" {randomName}.txt')
    print(f'chmod {sub} {randomName}.txt')
    os.system(f'chmod {sub} {randomName}.txt')