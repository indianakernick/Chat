<template>
  <template v-if="loaded">
    <div>
      <Message
        v-for="message in messages"
        :timestamp="message.timestamp"
        :userInfo="message.userInfo"
        :content="message.content"
        :sending="message.sending"
      />
    </div>
  </template>
  <StatusMessage v-else :status="status"/>
</template>

<script>
import Message from "./Message.vue";
import StatusMessage from "./StatusMessage.vue";

// For plain javascript files, the convention seems to be putting them in
// assets/js/

export default {
  name: "MessageList",

  components: {
    Message,
    StatusMessage
  },

  props: {
    userInfo: Object,
    userInfoCache: Object
  },

  data() {
    return {
      messages: [],
      status: "Loading...",
      loaded: false
    }
  },

  methods: {
    recentMessage(message) {
      this.messages.push({
        timestamp: message.timestamp,
        userInfo: this.userInfoCache.getUserInfo(message.author),
        content: message.content,
        sending: false
      });
    },

    messageReceipt(message) {
      // All messages arrive in the same order that they are sent.
      for (const msg of this.messages) {
        if (msg.sending) {
          msg.sending = false;
          msg.timestamp = message.timestamp;
          return;
        }
      }
      console.error("\"message receipt\" but all messages have been sent");
    },

    recentMessageList(message) {
      this.messages = message.messages.map(msg => {
        return {
          timestamp: msg.timestamp,
          userInfo: this.userInfoCache.getUserInfo(msg.author),
          content: msg.content,
          sending: false
        };
      });
      this.loaded = true;
    },

    sendMessage(content) {
      this.messages.push({
        // Initial "guess" for the timestamp.
        // This will be updated by the server.
        timestamp: new Date().valueOf() / 1000,
        userInfo: this.userInfo,
        content: content,
        sending: true
      });
    },

    createEmpty() {
      this.loaded = true;
    }
  }
};
</script>

<style>

</style>
