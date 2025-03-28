#!/usr/bin/env python3

import os
import pathlib
import shutil

BLOCK_SIZE = 131072  # 1 MiB
NUM_BLOCKS = 1000
FILE_SIZE = 4096
NUM_FILES = 250
FLAG = "utflag{d3l3t3d_bu7_n0t_g0n3_4ever}"
DISK = pathlib.Path("disk.img")
MOUNT = pathlib.Path("mnt")

# cleanup last run
shutil.rmtree(MOUNT, ignore_errors=True)
shutil.rmtree(DISK, ignore_errors=True)

# create empty disk image
with open("/dev/zero", "rb") as f:
    with open(DISK, "wb") as g:
        g.write(f.read(NUM_BLOCKS * BLOCK_SIZE))
# add ext4 filesystem
os.system(f"mkfs.btrfs {DISK}")

# mount disk image
os.system(f"mkdir -p {MOUNT}")
os.system(f"sudo mount {DISK} {MOUNT}")
os.system(f"sudo chown -R $USER {MOUNT}")
os.system(f"sudo chgrp -R $USER {MOUNT}")

# create flag file
random_flag_file = os.urandom(16).hex()
with open(MOUNT / f"{random_flag_file}.txt", "wb") as f:
    hex_flag = FLAG.encode().hex()
    padded = (hex_flag + os.urandom(FILE_SIZE).hex())[:FILE_SIZE * 2]
    f.write(bytes(padded, "utf-8"))
os.sync()
print("Flag file:", random_flag_file)

# create a bunch of files
for i in range(0, NUM_FILES):
    random_name = os.urandom(16).hex()
    print(i, random_name)
    with open(MOUNT / f"{random_name}.txt", "wb") as g:
        g.write(bytes(os.urandom(FILE_SIZE).hex(), "utf-8"))
    os.sync()

# delete flag file
os.remove(MOUNT / f"{random_flag_file}.txt")
os.sync()

# cover our tracks in the log some
for i in range(0, NUM_FILES):
    random_name = os.urandom(16).hex()
    print(i, random_name)
    with open(MOUNT / f"{random_name}.txt", "wb") as g:
        g.write(bytes(os.urandom(FILE_SIZE).hex(), "utf-8"))
    os.remove(MOUNT / f"{random_name}.txt")
    os.sync()

# unmount disk image
os.system(f"sudo umount {MOUNT}")
os.system(f"rmdir {MOUNT}")
print("Flag file:", random_flag_file)
