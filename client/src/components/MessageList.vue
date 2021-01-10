<template>
  <div v-if="loaded && messages.length > 0">
    <Message
      v-for="message in messages"
      :key="message.message_id"
      :timestamp="message.timestamp"
      :userInfo="message.userInfo"
      :content="message.content"
      :sending="message.sending"
    />
  </div>
  <StatusMessage v-else :status="status"/>
</template>

<script>
import Message from "./Message.vue";
import StatusMessage from "./StatusMessage.vue";
import { DELETED_USER_INFO } from "./Message.vue";

const MESSAGE_LIST_LIMIT = 50;

export default {
  name: "MessageList",

  components: {
    Message,
    StatusMessage
  },

  props: {
    userInfo: Object,
    userInfoCache: Object,
    shown: Boolean
  },

  data() {
    return {
      messages: [],
      status: "Loading...",
      loaded: false,
      loadingOld: false,
      oldest: 0
    }
  },

  methods: {
    initializeMessage(message) {
      return {
        message_id: message.message_id,
        timestamp: message.timestamp,
        userInfo: this.userInfoCache.getUserInfo(message.author),
        content: message.content,
        sending: false
      };
    },

    recentMessage(message) {
      this.messages.push(this.initializeMessage(message));
      if ((!this.shown || document.visibilityState === "hidden") && Notification.permission === "granted") {
        const userInfo = this.userInfoCache.getUserInfo(message.author);
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
          msg.message_id = message.message_id;
          msg.timestamp = message.timestamp;
          return;
        }
      }
      console.error("\"message receipt\" but all messages have been sent");
    },

    recentMessageList(messages) {
      this.messages = messages.map(this.initializeMessage);
      if (this.messages.length === 0) {
        this.setNoMessageStatus();
      }
      this.loaded = true;
    },

    oldMessageList(messages) {
      if (messages.length < MESSAGE_LIST_LIMIT) {
        this.oldest = this.messages[0].message_id;
      } else {
        const length = messages.length;
        for (let i = 0; i !== length; ++i) {
          this.messages.splice(i, 0, this.initializeMessage(messages[i]));
        }
      }
      this.loadingOld = false;
    },

    oldestMessage() {
      if (!this.loaded) return 0;
      if (this.loadingOld) return 0;
      if (this.oldest !== 0) return 0;
      if (this.messages.length < MESSAGE_LIST_LIMIT) return 0;
      if (this.messages[0].sending) return 0;
      this.loadingOld = true;
      return this.messages[0].message_id;
    },

    sendMessage(content) {
      this.messages.push({
        message_id: 0,
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
