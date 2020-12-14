<template>
  <template v-if="loaded">
    <div id="message-list">
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
import Message, { DELETED_USER_INFO } from "./Message.vue";
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
    userInfo: Object
  },

  data() {
    return {
      messages: [],
      status: "Loading...",
      loaded: false,
      userInfoCache: {
        0: DELETED_USER_INFO,
        [USER_ID]: this.userInfo
      }
    }
  },

  methods: {
    getUserInfo(id) {
      if (!this.userInfoCache.hasOwnProperty(id)) {
        this.userInfoCache[id] = {
          name: "",
          picture: ""
        };

        const req = new XMLHttpRequest();

        req.onload = () => {
          this.userInfoCache[id].name = req.response.name;
          this.userInfoCache[id].picture = req.response.picture;
        };

        req.responseType = "json";
        req.open("GET", `/api/user/${id}`);
        req.send();
      }

      return this.userInfoCache[id];
    },

    recentMessage(message) {
      const userInfo = this.getUserInfo(message.author);
      this.messages.push({
        timestamp: message.timestamp,
        userInfo: userInfo,
        content: message.content,
        sending: false
      });
    },

    messageReceipt(message) {
      // All messages arrive in the same order that they are sent.
      // Also, I think for-in has an undefined order.
      // Not sure about for-of though
      const messages = this.messages
      const length = messages.length;
      for (let idx = 0; idx !== length; ++idx) {
        if (messages[idx].sending) {
          messages[idx].sending = false;
          messages[idx].timestamp = message.timestamp;
          return;
        }
      }
      console.error("\"message receipt\" but all messages have been sent");
    },

    recentMessageList(message) {
      this.messages = message.messages.map(msg => {
        return {
          timestamp: msg.timestamp,
          userInfo: this.getUserInfo(msg.author),
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
    }
  }
};
</script>

<style scoped>

</style>
