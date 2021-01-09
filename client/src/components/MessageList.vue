<template>
  <template v-if="loaded && messages.length > 0">
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
import { DELETED_USER_INFO } from "./Message.vue";

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
      const userInfo = this.userInfoCache.getUserInfo(message.author);
      this.messages.push({
        timestamp: message.timestamp,
        userInfo: userInfo,
        content: message.content,
        sending: false
      });
      if (document.visibilityState === "hidden" && Notification.permission === "granted") {
        let notif;
        if (userInfo.name.length > 0) {
          notif = new Notification(userInfo.name, {
            body: message.content,
            icon: userInfo.picture48
          });
        } else {
          notif = new Notification("", {
            body: message.content
          });
        }
        notif.onclick = () => window.focus();
      }
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
      if (this.messages.length === 0) {
        this.setNoMessageStatus();
      }
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
      this.setNoMessageStatus();
      this.loaded = true;
    },

    setNoMessageStatus() {
      this.status = "This channel has no messages";
    },

    deleteUser(userId) {
      const deleted = this.userInfoCache.getUserInfo(userId);
      for (const message of this.messages) {
        if (message.userInfo === deleted) {
          message.userInfo = DELETED_USER_INFO;
        }
      }
    }
  }
};
</script>

<style>

</style>
