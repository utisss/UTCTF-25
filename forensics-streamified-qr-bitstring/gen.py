#!/usr/bin/env python3

import qrcode
import numpy as np

FLAG = "utflag{b!t_by_b!t}"
TEXT_FILE = "bitstring.txt"
IMAGE_FILE = "qr.png"

# Generate QR code with no border
qr = qrcode.QRCode(border=0)
qr.add_data(FLAG)
qr.make(fit=True)

# save image to file
image = qr.make_image(fill_color="black", back_color="white")
image.save(IMAGE_FILE)

# Extract raw QR matrix (1-bit per module)
qr_matrix = np.array(qr.modules, dtype=int)  # Each module is already 0 or 1

# Convert to 1D bitstring (row-major order)
bitstring = ''.join(str(bit) for row in qr_matrix for bit in row)

# Write to file
with open(TEXT_FILE, "w") as f:
    f.write(bitstring)

print(f"QR bitstring saved to {TEXT_FILE}")
print(f"QR dimensions: {qr_matrix.shape[0]} x {qr_matrix.shape[1]}")
