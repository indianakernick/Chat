<template>
  <div id="message-list">
    <!-- Should use key with v-for. Forgot why... -->
    <Message
        v-for="message in messages"
        :content="message.content"
        :creation_time="message.creation_time"
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
    const req = new XMLHttpRequest();
    req.addEventListener("error", () => {
      this.messages = [];
    });
    req.addEventListener("load", () => {
      this.messages = req.response.map(res => {
        res.sending = false;
        return res;
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
      console.log("Connection error");
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
            creation_time: message.timestamp,
            sending: false
          });
          break;

        case "message sent":
          // TODO: but which message was sent?
          for (const idx in this.messages) {
            if (this.messages[idx].sending) {
              this.messages[idx].sending = false;
              this.messages[idx].creation_time = message.timestamp;
            }
          }
          this.messages = this.messages.slice();
          break;
      }
    },

    sendMessageContent(message) {
      // TODO: Timestamps
      this.messages.push({
        content: "<You>: " + message,
        // Initial "guess" for the send time.
        // This will be updated by the server.
        creation_time: new Date().valueOf() / 1000,
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
