<template>
  <template v-if="connected">
    <div id="message-list">
      <!-- Should use key with v-for. Forgot why... -->
      <Message
          v-for="message in messages"
          :timestamp="message.timestamp"
          :author="message.author"
          :content="message.content"
          :sending="message.sending"
          :picture="message.picture"
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
import Message from "./Message.vue";
import StatusMessage from "./StatusMessage.vue";

const INITIAL_RETRY_DELAY = 125;
const MAX_RETRY_DELAY = 16000;

export default {
  name: "MessageList",

  components: {
    Message,
    StatusMessage
  },

  props: {
    name: String,
    picture: String
  },

  data() {
    return {
      messages: [],
      connected: false,
      status: "Connecting...",
      retryDelay: INITIAL_RETRY_DELAY
    }
  },

  created() {
    this.openConnection();
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
      this.socket = new WebSocket(`wss://${window.location.host}/api/socket`);
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
          this.messages.push({
            timestamp: message.timestamp,
            author: message.author.toString(),
            picture: "",
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
              author: msg.author === user_id ? this.name : msg.author.toString(),
              picture: msg.author === user_id ? this.picture : "",
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
        author: this.name,
        picture: this.picture,
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
