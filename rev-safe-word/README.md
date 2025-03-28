# Safe Word

Ah, the bugbear of every person who has written C. Segfault! (with a side of sigill).

This challenge was designed so that it would not be trivial to solve via side-channel (you can still do it,
but it will require some actual reverse engineering).

## Overview

The challenge implements a very convoluted finite automata. Nodes in the graph turn into ranges
of 256 longs in the array. Edges are represented as indexes into the segment of the node's array
which contain valid instructions. The goal is to produce a sequence of characters which will reach the
node where there is an edge labelled with 0.

## Okay, but what hints should I give?

Here is the rough path I imagine solvers will take:
- Observe general structure of program: It will load an index from the array based on yours and
previous inputs, and execute it as instructions
- dump all indexes in array that have valid instructions
- observe that each instruction follows the general pattern of "load smthg into rax, ret"
- observe that next set of 256 indexes is selected based on what value of rax the previous instruction
loaded
- observe that all characters of your flag must correspond to valid indexes that will lead to other
valid indexes

from there, you can probably do a backtracking search for indexes.

## This sounds like a lot of guessing

Most of it can be found via statc + dynamic analysis of the binary.

## This seems too hard

It is intended to stop teams from full-clearing rev in < 6 hours.
If somebody is a beginner and struggling with this challenge, tell them to try some of the earlier
revs.
