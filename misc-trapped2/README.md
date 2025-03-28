# Trapped in Plain Sight 2

* **Event:** UTCTF
* **Problem Type:** Misc
* **Tools Required / Used:** SSH Client

## Steps

1. Login into the server using the provided credentials.
2. Run `ls` to list the files in the directory.
3. Run `cat flag.txt` to get the flag. See that this produces a permission denied error.
4. Run `ls -l` to see the permissions of flag.txt. Notice that the permissions have a + sign at the end, meaning there is an ACL set on the file.
5. Run `getfacl flag.txt` to see the ACL set on the file. Notice that the ACL allows the user "secretuser" to read the file.
6. You may try to ssh as "secretuser", but you will need the password.
7. Read the passwd file by running `cat /etc/passwd` and notice that the password "hunter2" is stored as a comment associated with "secretuser.
8. Either `su secretuser` or logout and ssh as secretuser with the password "hunter2".
9. Run `cat flag.txt` to get the flag. (Or `cat /home/trapped/flag.txt` if you are not in the home directory.)

## Suggested Hints

* What are the permissions of the file? Who owns it?
* What are other ways of restricting access to a file besides the file permission bits?
* What are ways to gain access to a file you don't have permission to read?
* How can we find out what ACLs are set on a file?
* How can we access the system as another user?
