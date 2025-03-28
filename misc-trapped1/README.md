# Trapped in Plain Sight 1

* **Event:** UTCTF
* **Problem Type:** Misc
* **Tools Required / Used:** SSH Client

## Steps

1. Login into the server using the provided credentials.
2. Run `ls` to list the files in the directory.
3. Run `cat flag.txt` to get the flag. See that this produces a permission denied error.
4. Run `ls -l` to see the permissions of flag.txt. Notice that it can only be read by the owner, which is a user called "noaccess".
5. Now we need to look for a SUID program that can read the file for us. Run `find / -perm -u=s -type f 2>/dev/null` to find all SUID programs on the system.
6. Notice that among the results which are mostly user login control programs, there is a program called `xxd`. This is the program we need to use to read the file.
7. Run `xxd flag.txt` to get the flag.

## Suggested Hints

* What are the permissions of the file? Who owns it?
* What are ways to gain access to a file you don't have permission to read?
* What is the SUID bit and how does it work? How can we find SUID programs on a system?
* What are some tools we use to read a text file?
