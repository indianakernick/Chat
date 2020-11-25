<template>
  <div id="message-list">
    <Message
        v-for="message in messages"
        v-bind:content="message.content"
        v-bind:creation_time="message.creation_time"
    ></Message>
  </div>
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
  }
}
</script>

<style scoped>

</style>