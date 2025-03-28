# Streamified

* **Event:** UTCTF
* **Problem Type:** Forensics
* **Tools Required / Used:** QR code reader

## Steps

1. The first step is trying to figure out what the bitstring is meant to be. Based on the name and description of the challenge, you can figure out that a) you have to deserialize it and b) it is something like a barcode or QR code.
2. Counting the length of the bitstring, we see that it is 625 bits long. Since 625 is a perfect square, we can guess that the bitstring is meant to be a 25x25 QR code.
3. Now we have to actually convert it to a QR code so we can read it. I chose to write a Python script using numpy and PIL, but you could possibly get away with just pasting the bitstring into a text editor, replacing the 0's with spaces, and replacing the 1's with some character that is easy to see (like `#`).
4. Once you have the QR code, you can use any QR code reader to read it and get the flag.

## Suggested Hints

* What do you think the bitstring is meant to be? What is something that can be scanned, as the description suggests?
* How can you convert a bitstring to a QR code?
