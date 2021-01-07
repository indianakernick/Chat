- [ ] **[1]** Report
    - Describe this whole project. The challenges and problems faced, how I
      overcame those challenges, the skills I learnt, how I applied those 
      skills, some things I might do better next time (better planning from the
      start and also keeping notes of my progress).
    - This has a high priority but it must be completed last.
- [ ] **[1]** Documentation
    - Describe how the app works. The technical details that make it function.
    - A low hanging fruit is the ER diagram.
    - This may end up being part of the report or be a separate document.
- [x] **[2]** Create groups
    - A user can create a new group, setting a unique name and a picture.
- [x] **[2]** Invite users to groups
    - Any member of a group may invite other users to that group.
    - An invitation is a link. Anyone with the link can join the group from
      within 24 hours of the link being created.
- [x] **[2]** Navigate between groups
    - Once the user is within one group, how do they go to another group? Adopt
      the discord approach and have a list down the side of the page.
- [ ] **[3]** Refactoring
    - There are a handful of TODOs relating to code quality and cleanliness.
    - The socket code on both the client and server is getting worse with every
      new feature.
- [ ] **[3]** User interface polish
    - As of writing, the UI is kind of ugly and boring.
    - Remember that Bootstrap isn't meant to solve everything. You'll still need
      to write a lot of CSS yourself.
    - Progress has been made but the login page and also the dialog boxes need
      some work.
- [ ] **[3]** Handle *lots* of messages
    - Currently, the user isn't able to scroll back past the 50 most recent
      messages.
    - Older messages should be removed as newer messages appear.
- [ ] **[4]** Handle disconnected state better
    - Maybe a show a countdown when the client is trying to reconnect.
    - Maybe hide everything while disconnected.
    - Switching between groups involves disconnecting briefly so that will
      need to be considered.
    - A related issue is handling errors. If something unexpected happens like a
      a database error then we should show a generic "something went wrong"
      message.
- [x] **[4]** Rename channels
    - Any member of a group can rename channels within the group.
    - The new channel name must be unique within the group.
- [ ] **[4]** Rename groups (including changing the picture)
    - Any member of a group can change the name or picture of that group.
    - The new name must be unique.
- [ ] **[4]** Rename users (including changing the picture)
    - A user can change their name or picture.
    - The new name must be unique.
- [ ] **[5]** Delete messages
    - A user can select a message that they wrote and delete it
    - Messages from deleted users cannot be deleted because no-one owns them.
      Although you could say that anyone can delete them...
- [ ] **[5]** Edit messages
    - A user can select a message that they wrote and edit it.
    - Messages from deleted users cannot be edited because no-one owns them.
- [ ] **[5]** Delete user
    - All of a user's messages would need to be updated on each client. This
      would probably use the same mechanism as the "edit message" feature so if
      we implement one of these features, we should implement them both.
- [ ] **[5]** Delete group
    - This will delete all messages and forcefully kick all clients.
- [ ] **[5]** Leave group
    - There could be multiple clients logged into the same user so all clients
      would need to be forcefully kicked.
    - The user's messages in the group will remain intact.
- [x] **[5]** Logout
    - This would essentially involve deleting the session cookie and then load
      the login page.
    - The server only needs to set the expiration date of the cookie to some
      date in the past.
- [x] **[5]** Simple notifications
    - If the page is loaded but not visible, show a notification when a message
      arrives.
    - [`Notification on mdn`](https://developer.mozilla.org/en-US/docs/Web/API/Notifications_API/Using_the_Notifications_API).
    - [`Notification standard`](https://notifications.spec.whatwg.org/)
- [ ] **[6]** Background notifications
    - Users should get desktop notifications for messages sent in all groups
      that they’re a member of.
    - We'd need to use a service worker to maintain an SSE connection. The whole
      thing would be completely separate from the web socket code that we have.
- [ ] **[6]** Full featured notifications
    - To really implement this feature properly, we would probably need to add
      ability to mention users and configure notifications (e.g. just get
      notifications for mentions).
- [ ] **[6]** Binary format to replace JSON
    - Using a binary format would improve server performance and reduce
      bandwidth usage.
    - This might increase client complexity and it's unclear whether this will
      have a positive or negative impact on client performance.
    - It's a lot of work for an improvement that's difficult to see.
    - Some relevant resources are
      [`TextEncoder`](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder)
      and
      [`TextDecoder`](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder)
- [ ] **[6]** Host images
    - Host group and user images on the server instead of just storing URLs.
    - The server would take images from a source, resize them to the appropriate
      size, optimize them, and composite translucent images onto a solid
      background.
    - This also means that we can set the cache control headers and allow users
      to upload their own images (instead of using a URL).
    - The client would be simplified because it wouldn't need to worry about
      what the image URLs actually are. The image URL could be derived directly
      from the user ID or group ID.
    - So that the server doesn't need to worry about resizing and compositing
      images, we would probably want to do that on the client. That's pretty
      easy to do in JavaScript. Image optimization should be left to the server.
    - A real website would use a CDN but those cost money and this is a pet
      project so who cares.
    - **Tried it. Turns out not to be such a great idea...** See 2021-01-07.md. Was previously at
      **[3]**.

Priority **[6]** probably won't be implemented. Notifications just seems like a
massive pain.
