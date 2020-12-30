<template>
  <div class="message-sender">
    <!--
    TODO: Maybe use textarea instead of div.
    I'm using a div because it's easier to move it where I want it. Also, a
    div will grow to fit its content. For a textarea, that requires some work.
    https://stackoverflow.com/questions/454202/creating-a-textarea-with-auto-resize/5346855#5346855
    Using a div means that the character count is more difficult to enforce.
    There's also a bunch of other quirks to deal with like the trailing <br> and
    the placeholder text.
    -->
    <div
      class="message-box"
      :placeholder="'Message #' + currentChannelName"
      :contenteditable="connected"
      aria-multiline="true"
      @keypress.enter="pressEnter"
      @input="input"
      @paste="paste"
    />
    <div ref="countElement" class="character-count">{{ characterCount }} / 1024</div>
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
    pressEnter(e) {
      if (!e.shiftKey) {
        e.preventDefault();
        let text = e.target.innerText;
        if (text.endsWith("\n")) {
          text = text.substring(0, text.length - 1);
        }
        if (text.length !== 0) {
          this.$emit("sendMessage", e.target.innerText);
        }
        e.target.innerHTML = "";
      }
    },

    input(e) {
      if (e.target.innerHTML === "<br>") {
        e.target.innerHTML = "";
      }
      const text = e.target.innerText;
      this.characterCount = text.length - text.endsWith("\n");
      // TODO: Enforce character limit
      // This actually isn't as simple as it might seem.
      // https://stackoverflow.com/questions/33551502/set-max-length-for-content-editable-element
      // https://github.com/antpv/contenteditable-max-length/blob/master/src/contenteditableMaxLength.js

      // There's probably a more efficient way of doing this. Maybe do some
      // timing in JavaScript and only use CSS for the actual fading effect.
      // https://css-tricks.com/restart-css-animation/#update-another-javascript-method-to-restart-a-css-animation
      this.$refs.countElement.classList.remove("hide-animation");
      void this.$refs.countElement.offsetWidth;
      this.$refs.countElement.classList.add("hide-animation");
    },

    // Overriding the paste event to prevent pasting with formatting.
    paste(e) {
      e.preventDefault();
      const text = e.clipboardData.getData("text/plain");
      document.execCommand("insertText", false, text);
    }
  }
};
</script>

<style>
.message-sender {
  background-color: dimgray;
  border-top: 1px solid black;
}

.message-box {
  margin: 8px;
  background-color: gray;
  color: gainsboro;
  border-radius: 4px;
  padding: 4px 8px 4px 8px;
  overflow-wrap: anywhere;
}

.message-box:empty:not(:focus):before {
  content: attr(placeholder);
  pointer-events: none;
  color: silver;
}

.message-box:focus {
  outline: none;
}

.character-count {
  float: right;
  font-size: 0.6rem;
  color: white;
  position: absolute;
  right: 8px;
  bottom: 8px;
  padding: 1px 2px 1px 2px;
  border-radius: 2px;
  background-color: #6969697F; /* dimgray */
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
