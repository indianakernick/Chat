<template>
  <div id="message-list">
    <!-- Should use key with v-for. Forgot why... -->
    <Message
        v-for="message in messages"
        :timestamp="message.timestamp"
        :author="message.author"
        :content="message.content"
        :sending="message.sending"
    ></Message>
  </div>
  <input
      type="text"
      @keypress.enter="sendMessageContent($event.target.value), $event.target.value = ''"
  />
</template>

<script>
import Message from "./Message.vue";

export default {
  name: "MessageList",

  components: {
    Message
  },

  data() {
    return {
      messages: []
    }
  },

  created() {
    this.socket = new WebSocket(`ws://${window.location.host}/api/socket`);

    this.socket.addEventListener("message", this.receiveMessage);

    this.socket.addEventListener("error", () => {
      // TODO: tell the user that an error occurred
      // Still print the error message to the console though
      console.error("Connection error");
    });

    this.socket.addEventListener("open", () => {
      this.socket.send('{"type":"request recent messages"}');
    });
  },

  methods: {
    receiveMessage(event) {
      console.log(event.data);
      const message = JSON.parse(event.data);
      switch (message.type) {
        case "error":
          // TODO: tell the user that an error occurred
          // Still print the error message to the console though
          console.error(message.message);
          break;

        case "recent message":
          this.messages.push({
            timestamp: message.timestamp,
            author: message.author,
            content: message.content,
            sending: false
          });
          break;

        case "message receipt":
          // All messages arrive in the same order that they are sent.
          const messages = this.messages;
          const length = messages.length;
          for (let idx = 0; idx !== length; ++idx) {
            if (messages[idx].sending) {
              messages[idx].sending = false;
              messages[idx].timestamp = message.timestamp;
              return;
            }
          }
          console.error("\"message sent\" but all messages have been sent");
          break;

        case "recent message list":
          // TODO: Should prevent sending "post message" until "recent message list" is received
          this.messages = message.messages.map(msg => {
            return {
              timestamp: msg.timestamp,
              author: msg.author,
              content: msg.content,
              sending: false
            };
          });
          break;
      }
    },

    sendMessageContent(content) {
      this.messages.push({
        // Initial "guess" for the timestamp.
        // This will be updated by the server.
        timestamp: new Date().valueOf() / 1000,
        author: -1,
        content: content,
        sending: true
      });
      this.socket.send(JSON.stringify({
        type: "post message",
        content: content
      }));
    }
  }
};
</script>

<style scoped>

</style>
