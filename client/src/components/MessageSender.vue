<template>
  <div class="message-sender">
    <!--
    SIMPLIFIED: A real app would probably use a div with contenteditable but
    it's really difficult to get it to behave nicely.
    https://medium.com/content-uneditable/contenteditable-the-good-the-bad-and-the-ugly-261a38555e9c
    -->
    <textarea
      aria-label="Message sender box"
      class="message-box"
      :placeholder="'Message #' + currentChannelName"
      :readonly="!connected"
      maxlength="1024"
      rows="1"
      @keypress.enter="enter"
      @input="input"
    />
    <div ref="count" class="character-count">{{ characterCount }} / 1024</div>
  </div>
</template>

<script>
export default {
  name: "MessageSender",

  props: {
    connected: Boolean,
    currentChannelName: String
  },

  emits: [
    "sendMessage"
  ],

  data() {
    return {
      characterCount: 0
    };
  },

  methods: {
    enter(e) {
      if (!e.shiftKey) {
        e.preventDefault();
        const box = e.target;
        if (box.value.length !== 0) {
          this.$emit("sendMessage", box.value);
          box.value = "";
          this.characterCount = 0;
        }
      }
    },

    input(e) {
      const box = e.target;
      box.style.height = "auto";
      box.style.height = box.scrollHeight + "px";
      this.characterCount = box.value.length;
      // https://css-tricks.com/restart-css-animation/#update-another-javascript-method-to-restart-a-css-animation
      this.$refs.count.classList.remove("hide-animation");
      void this.$refs.count.offsetWidth;
      this.$refs.count.classList.add("hide-animation");
    }
  }
};
</script>

<style lang="scss">
@import "../scss/colors";

.message-sender {
  border-top: 1px solid $column-title-border;
  position: relative;
}

.message-box {
  background-color: $sender-box-back;
  color: $sender-box-text;
  display: block;
  margin: 8px;
  padding: 4px 8px;
  width: calc(100% - 16px);
  border-radius: 4px;
  border: none;
  overflow-wrap: anywhere;
  resize: none;
}

.message-box::placeholder {
  color: $sender-placeholder-text;
}

.message-box:focus {
  outline: none;
}

.character-count {
  color: $char-count-text;
  background-color: $char-count-back;
  font-size: 0.6rem;
  position: absolute;
  right: 8px;
  bottom: 8px;
  padding: 1px 2px;
  border-radius: 2px;
  opacity: 0;
}

.hide-animation {
  animation: hideAnimation 5s forwards;
}

@keyframes hideAnimation {
  0%   { opacity: 1; }
  90%  { opacity: 1; }
  100% { opacity: 0; }
}
</style>
