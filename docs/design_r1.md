# Revised Design (2020-12-26)

## Concepts

### User
Users can create a new account or log into an existing account (both workflows
are identical) using Google sign in. Their account will be linked with their
Google account and inherit the name and profile picture from Google.

Users may choose to change their name and profile picture. The user’s name must
be unique. The profile picture is simply a URL.

### Group
A group is a collection of channels and users. The users are members of the
group and are able to add new members. This happens via an invitation process. A
member of a group can invite other users to be new members.

### Channel
A channel is a stream of messages. Users can post messages to a channel for
other users to read. Users can create, delete, or rename channels. The name of a
channel is limited in length and cannot contain certain characters. The names of
channels must also be unique within the group they’re contained in.

### Message
A message is a snippet of text written by a particular user. The message
contains the message content (the text itself) which has a limited size. A
message also keeps the date it was first created. Messages cannot be edited.

Messages from users that have deleted their accounts will be anonymized. The
message will remain as it was, but the name of the author will not be present.

## Summary of design changes

All rooms are private. 

## Progress (as of writing)
 - Users can sign-up/login.
 - Users can post messages to channels and read messages from other users.
 - Users can create or remove channels from groups.
