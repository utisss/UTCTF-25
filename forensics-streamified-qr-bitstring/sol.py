#!/usr/bin/env python3

import numpy as np
from PIL import Image

FILE = "bitstring.txt"

with open(FILE, "r") as f:
    bitstring = f.read().strip()

# Convert bitstring to 2D array
n = int(len(bitstring) ** 0.5)
qr_matrix = np.array([int(bit) for bit in bitstring], dtype=int).reshape(n, n)

# Convert to QR code
image = Image.fromarray((qr_matrix * 255).astype(np.uint8))
image.save("qr_decoded.png")