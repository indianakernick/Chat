<template>
  <div id="message-list">
    <!-- Should use key with v-for. Forgot why... -->
    <Message
        v-for="message in messages"
        v-bind:content="message.content"
        v-bind:creation_time="message.creation_time"
    ></Message>
  </div>
  <button v-on:click="sendHello">Send Hello</button>
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
    sendHello() {
      // TODO: Timestamps
      // TODO: Indicate where the message was received by the server
      this.messages.push({content: "<You>: Hello!", creation_time: 0});
      this.socket.send("Hello!");
    }
  }
};
</script>

<style scoped>

</style>
