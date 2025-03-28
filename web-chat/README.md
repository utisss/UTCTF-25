
## Chat Writeup

The core components of the challenge
- a basic chat room where users can send messages, create and delete channels
- starts out with three core channels
    - #general - the starting channel
    - #log - admin-only, can't join
    - #mod-only - mod-only, can't join

- Every 2 minutes, a "moderator" joins #general and announces the time
  (using a /announce command that can't be used without privileges)

- There's an XSS bug in the channel list in the sidebar, but it requires
  actively clicking on the link, so the moderator won't do it


- The general goal should be to get access to the #mod-only channel
  (spoiler: the flag is in the channel description)

- Run `/help` to see all commands

- `/user` and `/channel` show some detailed info about users or the current channel
  ```
  Channel 'general':
  - description: 
  - slowmode: 0.05
  - hidden: false
  - immutable: true
  - owner: 0000000000000
  - current users: 14
  - admin-only: false
  - mod-only: false
  - mode: normal
  ```
  ```
  User '0njkxbh18w6r7':
  - name: username
  - privileges: Privileges(CHANNEL_CREATE | CHANNEL_DELETE | MESSAGE_SEND | CHANNEL_MODIFY)
  - created: 2025-03-15T19:17:54Z[Etc/Unknown]
  - banned: N/A
  - style: "& .username { color: var(--palette-4); &::before, &::after { color: var(--fg) } }"
  ```

- The main interesting part is the `/set` command:
    - `/nick example` and `/set user.name example` change the current username
    - `/set` will list available property groups:
      ```
      Available property groups: channel, user
      ```
    - `/set channel`:
      ```
      Available channel properties: .description, .slowmode, .hidden, .immutable, .owner, .admin-only, .mode
      ```
    - `/set user`:
      ```
      Available user properties: .name, .style
      ```

The core channels are immutable, so they can't be messed with.
But if you create your own channel, you can experiment with what the
different channel properties do.

The core thing to notice is the existance of the log channel, and the
`mode` property on other channels of "normal".  If your try setting the
channel mode to something random, you'll get:
```
Invalid channel mode. Valid modes: 'normal', 'log'
```
Then, if you try to set it to log, you'll get:
```
Log channels must be hidden and admin-only.
```
So, you'll need to make the channel hidden (/set channel.hidden true)
and admin-only (/set channel.admin-only true).
There's only one problem with that -- setting the channel as admin-only
will kick you from the channel.

*However*, it doesn't do it immediately -- if you use a script or
the console to send multiple messages in series, most of the time
the other messages go through before you get kicked.  As such, if
you set the channel admin-only, set the mode to log, and set the channel
*back to non admin-only*, you'll get kicked, but then you can rejoin
afterwards.

Then, you have access to a log channel, which shows all messages sent
in every channel.  Now, the main thing to look for is what other things
are special cased -- basically, look at what the Moderator bot is doing.

Specifically, the moderator joins general every two minutes, and then
sends an announcement -- what commands does it run?

```
/join
/login some-password-here-asdf
/announce Announcement: the current time is ....
/leave
```

(Note that the excerpt above isn't the password, but it's the same format.)

So the password for login is logged in plain text, which allows you
to log in as a moderator and gain access to #mod-info.

The flag is in the channel description of #mod-info.

## Partial solve script

```
// set up socket...

for (let msg of [
    "/create _temp",
    "/join _temp",
    "/set channel.admin-only true",
    "/set channel.mode log",
    "/set channel.admin-only false",
]) socket.send(msg)
```

If the script succeeds, then join _temp, and you should see the log.
