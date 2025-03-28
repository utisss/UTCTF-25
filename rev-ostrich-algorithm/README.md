# Ostrich Algorithm

## Overview

There are a lot of potential solutions to this challenge. Three that I can think of:
- Patch the constant buffer to match the message
- set breakpoint right before the checking logic begins and set $rip to skip the check
- Write a gdb-script to cheat the check (breakpoint at the check, set al=dl, then continue)

## Possible Hints

The challenge is trivial if you have played a CTF before and done rev. If somebody is struggling
with this challenge, I'd guess that it's because they don't know where to start and/or aren't used
to thinking around the box.

So, possible hints:
- What is a disassembler? Have you opened the binary up in a disassembler/decompiler?
- Maybe debugging binary will help?
- Can you think of a way to modify the binary to pass the check?
- Can you modify register values at certain points in the binary?
