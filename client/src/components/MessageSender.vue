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
  /*
  form-control height is set to this calc expression.
  Duplicating it here seems a bit fragile but I'm not sure what else to do.
  Could maybe use SCSS somehow...
  */
  min-height: calc(1.5em + 0.75rem + 2px);
  height: auto;
}
</style>
