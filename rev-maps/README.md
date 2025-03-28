# Transformer

Because no mid-level CTF would be complete without a side-channel problem!

## Overview

Intended solution is side-channel. Observe that each a character of the input only affects certain characters of the output,
independent of other characters. Thus, one could write a script that tries each character. Characters
that yield an output that is closer(read: has more of the same characters in the same places) to
the intended output are closer to the flag. This script would slowly build the flag, returning the
correct answer.

But, it's kinda lame to tell somebody "Try different outputs, what do you observe?" (even though developing the ability
to make clever guesses/observations based on external program behavior is a very useful skill!).

## What about actual reverse engineering?

Ghidra/Binary ninja can guess at destructor names. A couple of functions are repeated throughout most
of the program, most of which are boilerplate. Identify which ones are boilerplate, rename accordingly,
and then concentrate on the actual computation.

Note: even with this approach, it will still be very time-consuming to reverse the entire binary.
