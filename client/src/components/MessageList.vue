<template>
  <div id="message-list">
    <!-- Should use key with v-for. Forgot why... -->
    <Message
        v-for="message in messages"
        :content="message.content"
        :creation_time="message.creation_time"
    ></Message>
  </div>
  <input
      type="text"
      @keypress.enter="sendMessage($event.target.value), $event.target.value = ''"
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
      this.messages = req.response;
    });
    req.responseType = "json";
    req.open("GET", "/api/messages");
    req.send();

    // TODO: error checking
    // TODO: Timestamps. Can we trust the client to give us accurate timestamps?
    this.socket = new WebSocket(`ws://${window.location.host}/api/socket`);
    this.socket.addEventListener("message", e => {
      this.messages.push({content: e.data, creation_time: 0});
    });
  },

  methods: {
    sendMessage(message) {
      // TODO: Timestamps
      // TODO: Indicate where the message was received by the server
      this.messages.push({content: "<You>: " + message, creation_time: 0});
      this.socket.send(message);
    }
  }
};
</script>

<style scoped>

</style>
