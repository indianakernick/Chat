<template>
  <div
    class="form-control"
    :contenteditable="connected"
    aria-multiline="true"
    @keypress.enter="pressEnter"
  ></div>
</template>

<script>
export default {
  name: "MessageSender",

  props: {
    connected: Boolean
  },

  emits: [
    "sendMessage"
  ],

  methods: {
    pressEnter(e) {
      if (!e.shiftKey) {
        e.preventDefault();
        if (e.target.innerText.length !== 0) {
          this.$emit("sendMessage", e.target.innerText);
        }
        e.target.innerHTML = "";
      }
    },
  }
};
</script>

<style>
div.form-control {
  height: unset;
}
</style>
