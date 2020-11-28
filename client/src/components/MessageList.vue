<template>
  <div id="message-list">
    <!-- Should use key with v-for. Forgot why... -->
    <Message
        v-for="message in messages"
        :content="message.content"
        :timestamp="message.timestamp"
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
    // Should probably get the initial messages from the socket
    const req = new XMLHttpRequest();
    req.addEventListener("error", () => {
      this.messages = [];
    });
    req.addEventListener("load", () => {
      this.messages = req.response.map(res => {
        return {
          content: res.content,
          timestamp: res.creation_time,
          sending: false
        };
      });
    });
    req.responseType = "json";
    req.open("GET", "/api/messages");
    req.send();

    this.socket = new WebSocket(`ws://${window.location.host}/api/socket`);

    this.socket.addEventListener("message", this.receiveMessage);

    this.socket.addEventListener("error", () => {
      // TODO: tell the user that an error occurred
      // Still print the error message to the console though
      console.error("Connection error");
    });

    // TODO: only use the socket after the connection has opened
  },

  methods: {
    receiveMessage(event) {
      console.log(event.data);
      const message = JSON.parse(event.data);
      switch (message.type) {
        case "error":
          // TODO: tell the user that an error occurred
          // Still print the error message to the console though
          console.log(message.message);
          break;

        case "new message":
          this.messages.push({
            content: `<${message.from}>: ${message.content}`,
            timestamp: message.timestamp,
            sending: false
          });
          break;

        case "message sent":
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
      }
    },

    sendMessageContent(message) {
      this.messages.push({
        content: "<You>: " + message,
        // Initial "guess" for the send time.
        // This will be updated by the server.
        timestamp: new Date().valueOf() / 1000,
        sending: true
      });
      this.socket.send(JSON.stringify({
        type: "send message",
        content: message
      }));
    }
  }
};
</script>

<style scoped>

</style>
