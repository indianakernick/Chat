<template>
  <input
    type="text"
    @keypress.enter="pressEnter($event.target)"
  />
</template>

<script>
export default {
  name: "MessageSender",

  emits: [
    "sendMessage"
  ],

  methods: {
    pressEnter(input) {
      this.$emit("sendMessage", input.value);
      input.value = "";
    }
  }
};
</script>

<style scoped>

</style>