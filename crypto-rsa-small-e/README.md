Recall in RSA, a message $m$ is encrypted as follows:

$c = m^e \pmod n$

where $e$ and $n$ are part of the public key.

In this challenge, since $e$ is so small ($e=3$), and we don't pad $m$, we get that $m^e < n$. Thus

$c = m^e = m^3$

and we can recover $m$:

$m = c ^ {\frac{1}{3}}$.

After that it is just converting the integer to ascii :).