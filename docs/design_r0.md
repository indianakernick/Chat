# Design (2020-11-17)

## Concepts

### Accounts
Users can sign-up and log in. They’ll identify themselves using their email.
They can choose a display name and a password. Gravatar is used to assign a
profile picture. Emails and display names must be unique. Password and display
name can be changed. Not sure yet whether a third-party service should be used
for this (like Sign In With Google or whatever).
https://en.gravatar.com/site/implement/images/

Profiles are public and can be searched by name.

### Rooms
Users can send messages directly to each other via a direct message or in a room
with multiple users. In essence, direct messaging is a room with only two
members. Not sure if we should make that generalisation.

A room has a display name. The display name must be unique but can be changed. A
room also has a unique ID used to derive a unique URL. This ID never changes. A
room also has an image.

Rooms can either be public or private. A public room can be searched for by name
and anyone can request access to join them. A private room is invite-only.

#### Public rooms
Public rooms can be searched for by name. Anyone can request to join a public
room. Anyone can read the messages in a public room (but can’t post their own
messages unless granted access to join). When a user requests access to join a
public room, an administrator of the room has the responsibility of accepting or
denying the request. When the request is sent, all admins will receive a
notification. When the request is accepted, the user will receive a notification
and the room will appear in their list of rooms.

#### Private rooms
Private rooms are invite-only. They cannot be searched for. If their URL is used
but someone who doesn't have access, they’ll get a 404 page. Users can be
invited by searching for them by name. When invited, the user will get a
notification and the room will appear in their list of rooms.

#### Direct messaging
Direct messaging is similar to a private room except that the room can only ever
have two members. If the account of one of the members is destroyed, the room is
destroyed.

#### Roles
Members of a room have roles. The user that created the rooms is an
administrator and anyone else that joins is not an admin. Admins can change the
name and image for a room.

#### Messages
Messages can either be text or an image. The types of messages could be extended
with the chat-bot API to offer things like forms or surveys.

#### Notifications
Notifications are distributed as desktop notifications and through the iOS app
(if I decide to make it). Users can set notification settings on a per room
basis. For each room, they can decide if they receive notifications for all
messages, or just the messages that mention them, or none.

Email notifications are used for some things such as being accepted to join a
room.

## Use Cases

### View Home Page (not logged in)
If the user loads the home page and they aren't signed in, their only two
options are to create an account or log in to an existing account. They cannot
search for users or rooms. They cannot do anything until they create an account.

### View Home Page (logged in)
 - Search for accounts
 - Search for public rooms
 - See a list of rooms that the user is a member of
 - Log out
 - See account settings

### Create Account
The user can enter their email address, a display name, and a password. The
email address and display name must be unique. The email address must be
verified to be a valid address that is owned by the account holder. The display
name has a minimum and maximum length, and a restricted set of characters that
can be used. After successfully creating an account, they will be redirected to
the home page.

### Log In
The user will enter their email address, and their password. After successfully
logging in, they will be redirected to the home page.

### Enter Room (private room that hasn't been joined)
The user will get a 404 page if they’re trying to access a private room that
they’re not allowed to join.

### Create Room
The user must choose a unique display name. Optionally, they may choose an image
URL to use as the display image for the room. The user must also decide whether
this room should be public or private.

## Stages

### Stage 1
One single public room. Anybody can access it and send messages to it. Anybody
can read the messages from it. This also involves getting started with all the
tools and frameworks.

Not sure which framework to use for this.
https://www.lpalmieri.com/posts/2020-07-04-choosing-a-rust-web-framework-2020-edition/

### Stage 2
Accounts. Signup and login. Users are now identified in The One Room.

### Stage 3
Public rooms. Users can search for and join a public room. Joining doesn't
require requesting access, they can just join. Joining means that they’ll be
allowed to post messages and also receive notifications.

### Stage 4
Private rooms, direct messaging and roles. Public rooms now require that users
request to join.

still need server settings and notification settings

**Careful about feature creep. Don’t make something too big. Also don’t start
programming before the design is fleshed out. If you go for something too
complex, you’ll have to spend more time designing and less time programming. It
will also be more difficult to tell if you’ve reached your goal.**
