In admin-only and sudo there is a link to a Google doc and an admin password respectively.

In the google doc there is a link to a github gist, another password in the blacked out area, and the decryption key in the footer of page 2.

In the github gist's history there is a base64 encoded link to the mega.nz file which requires the decryption key.

The downloaded zip file requires one of the two passwords to open.

Reverting the git history changes the picture of the rabbit to a white one.

Using the other password (in the google doc) and steghide we can extract the flag



If someone asks for help with this problem, ask them what they have, see if they're going in the right direction