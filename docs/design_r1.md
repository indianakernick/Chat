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

## Prioritized tasks
 - **[1]** Report
   - Describe this whole project. The challenges and problems faced, how I
     overcame those challenges, the skills I learnt, how I applied those skills,
     some things I might do better next time (better planning from the start and
     also keeping notes of my progress).
   - This has a high priority but it must be completed last.
 - **[1]** Documentation
   - Describe how the app works. The technical details that make it function.
   - A low hanging fruit is the ER diagram.
   - This may end up being part of the report or be a separate document.
 - **[2]** Create groups
   - A user can create a new group, setting a unique name and a picture.
 - **[2]** Invite users to groups
   - Any member of a group may invite other users to that group.
   - An invitation is a link. Anyone with the link can join the group from
     within 24 hours of the link being created.
 - **[2]** Navigate between groups
   - Once the user is within one group, how do they go to another group? Adopt
     the discord approach and have a list down the side of the page.
 - **[3]** Refactoring
   - There are a handful of TODOs relating to code quality and cleanliness.
 - **[3]** User interface polish
   - As of writing, the UI is kind of ugly and boring.
   - Remember that Bootstrap isn't meant to solve everything. You'll still need
     to write a lot of CSS yourself.
 - **[4]** Rename channels
   - Any member a group can rename channels within the group.
   - The new channel name must be unique within the group.
 - **[4]** Rename groups (including changing the picture)
   - Any member of a group can change the name or picture of that group.
   - The new name must be unique.
 - **[4]** Rename users (including changing the picture)
   - A user can change their name or picture.
   - The new name must be unique.
 - **[5]** Notifications
   - Users should get desktop notifications for messages sent in groups that
     they’re a member of.
 - **[5]** Delete messages
   - A user can select a message that they wrote and delete it
   - Messages from deleted users cannot be deleted because no-one owns them.
 - **[5]** Edit messages
   - A user can select a message that they wrote and edit it.
   - Messages from deleted users cannot be edited because no-one owns them.

I might not implement **[5]**. Even **[4]** seem somewhat like nice-to-haves
rather than critical features.