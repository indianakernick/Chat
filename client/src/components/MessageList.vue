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

const CONNECTION_RETRY_TIME = 5000;

export default {
  name: "MessageList",

  components: {
    Message,
    StatusMessage
  },

  data() {
    return {
      messages: [],
      connected: false,
      status: "Connecting..."
    }
  },

  created() {
    this.openConnection();
    window.messageList = this;
  },

  methods: {
    initSocket() {
      this.socket = new WebSocket(`ws://${window.location.host}/api/socket`);
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
          setTimeout(this.retryConnection, CONNECTION_RETRY_TIME);
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
        setTimeout(this.retryConnection, CONNECTION_RETRY_TIME);
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
            author: message.author,
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
          this.connected = true;
          this.messages = message.messages.map(msg => {
            return {
              timestamp: msg.timestamp,
              author: msg.author,
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
        author: -1,
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
