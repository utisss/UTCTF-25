# Trapped in Plain Sight 1

* **Event:** UTCTF
* **Problem Type:** Forensics
* **Tools Required / Used:** Filesystem Recovery Tool

* note that CTFd picked up the git lfs object rather than the actual disk image, so I ended up uploading it to Google Drive instead

## Steps

1. Download the provided disk image.
2. Trying to mount it will reveal a directory with many files containing random data.
3. Since none of the files are the flag, this is a sign to try to look deeper into the mechanisms of the filesystem.
4. One place to look would be the metadata (this works for the other forensics chal, but not this one). Instead we will try to recover a deleted file.
5. I personally had a bunch of issues with Linux CLI tools, so I ended up using ReclaiMe File Recovery to recover the file. There are several other options out there, and the CLI tools may work for other people.
6. Once you have the filesystem, depending on the tool you used, either it may tell you which file has been deleted, or you may have to figure it out yourself:
    * One approach is to mount the disk image and compare the list of files in the mounted filesystem to the list of files in the recovered filesystem. The one file that is in the recovered filesystem but not in the mounted filesystem has the flag.
    * Another is to extract the contents of the files in the recovered filesystem and search for the flag format.
    * The file name for the flag is 4f48f18be818d84c946a4fe74a31d075.txt
7. The flag has been hex-encoded and padded at the end to blend in with the other files. Decode the first several characters to get the flag.

## Suggested Hints

* What are the contents of the disk image? (BTRFS filesystem)
* Why might we not be able to see the flag file?
* What tools can we use to recover deleted files?
* How can we identify the deleted flag file?
