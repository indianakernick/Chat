<template>
  <template v-if="connected">
    <div id="message-list">
      <!-- Should use key with v-for. Forgot why... -->
      <Message
          v-for="message in messages"
          :timestamp="message.timestamp"
          :userInfo="message.userInfo"
          :content="message.content"
          :sending="message.sending"
      ></Message>
    </div>
    <input
        type="text"
        @keypress.enter="pressEnter($event.target)"
    />
  </template>
  <StatusMessage v-else :status="status"></StatusMessage>
</template>

<script>
import Message, { DELETED_USER_INFO } from "./Message.vue";
import StatusMessage from "./StatusMessage.vue";

const INITIAL_RETRY_DELAY = 125;
const MAX_RETRY_DELAY = 16000;

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
      connected: false,
      status: "Connecting...",
      retryDelay: INITIAL_RETRY_DELAY,
      userInfoCache: {
        0: DELETED_USER_INFO,
        [USER_ID]: this.userInfo
      }
    }
  },

  created() {
    this.openConnection();
    // TODO: This is very useful for debugging but don't forget to remove it
    window.messageList = this;
  },

  methods: {
    getRetryDelay() {
      const delay = this.retryDelay;
      this.retryDelay = Math.min(MAX_RETRY_DELAY, 2 * this.retryDelay);
      return delay;
    },

    resetRetryDelay() {
      this.retryDelay = INITIAL_RETRY_DELAY;
    },

    initSocket() {
      this.socket = new WebSocket(`wss://${window.location.host}/api/socket/${CHANNEL_ID}`);
    },

    initListeners() {
      this.socket.onmessage = this.receiveMessage;

      this.socket.onerror = () => {
        this.connected = false;
      };

      this.socket.onclose = event => {
        this.connected = false;
        // 1000 means "normal closure"
        // https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent
        if (event.code !== 1000) {
          setTimeout(this.retryConnection, this.getRetryDelay());
        }
      };
    },

    openConnection() {
      this.initSocket();
      this.initListeners();

      this.socket.onopen = () => {
        this.socket.send('{"type":"request recent messages"}');
      };
    },

    retryConnection() {
      this.initSocket();

      this.socket.onerror = () => {
        setTimeout(this.retryConnection, this.getRetryDelay());
      };

      this.socket.onopen = () => {
        this.initListeners();
        this.socket.send('{"type":"request recent messages"}');
      };
    },

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

    receiveMessage(event) {
      console.log(event.data);
      const message = JSON.parse(event.data);
      switch (message.type) {
        case "error":
          console.error("Server error:", message.message);
          this.status = "An error has occurred";
          this.socket.close(1000);
          break;

        case "recent message":
          const userInfo = this.getUserInfo(message.author);
          this.messages.push({
            timestamp: message.timestamp,
            userInfo: userInfo,
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
          this.resetRetryDelay();
          this.connected = true;
          this.messages = message.messages.map(msg => {
            return {
              timestamp: msg.timestamp,
              userInfo: this.getUserInfo(msg.author),
              content: msg.content,
              sending: false
            };
          });
          break;
      }
    },

    pressEnter(input) {
      // Alternative might be to hide the text box until connected
      if (!this.connected) return;
      this.messages.push({
        // Initial "guess" for the timestamp.
        // This will be updated by the server.
        timestamp: new Date().valueOf() / 1000,
        userInfo: this.userInfo,
        content: input.value,
        sending: true
      });
      this.socket.send(JSON.stringify({
        type: "post message",
        content: input.value
      }));
      input.value = "";
    }
  }
};
</script>

<style scoped>

</style>
