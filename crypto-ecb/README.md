This is ECB but with a twist: the flag is inserted in the middle of the plaintext. This makes the problem a lot more annoying.

First step is to find the length of the flag, which can be done by adding bytes until the ct length changes (that will be a multiple of a block size)

Next, we want to set up a situation like this:

xxxxxxxxxxxxxxxFLAG_________________________????xxxxxxxxxxxxxxxy
|               |               |               |               

where we have 15 bytes of known input + the first byte of the flag, some other stuff, and then later down the line we have our same 15 bytes + a 16th known "guess" byte. Getting this just involves manipulating the "checksum" to force the algorithm to put the flag in the right spot (it's always possible since we can just tack random stuff onto the end of the string)

We can try all 256 possible bytes for y until we get a match, but more realistically we can just try printable ascii range since we know it's a flag

Next, repeat the process but with 14 bytes of known input + known first byte + unknown second byte, etc.

Once we run out of space, we can get past the 16 bytes of the flag by just using the next block:

xxxxxxxxxxxxxxxutflag{.........?????????????????????????????????tflag{.........y
|               |               |               |               |

If the flag were longer than 2 blocks we would need to keep doing this but fortunately it's not.