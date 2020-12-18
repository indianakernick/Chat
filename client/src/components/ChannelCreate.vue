<template>
  <form @submit.prevent="createChannel">
    <input
      type="text"
      maxlength="32"
      pattern="[^#@\p{White_Space}]+"
      @input="validate"
    />
  </form>
</template>

<script>
const REGEX_SPACE = /\p{White_Space}+/gu;
const REGEX_BAD_CHARS = /[#@]+/g;

export default {
  name: "ChannelCreate",

  emits: [
    "createChannel"
  ],

  data() {
    return {
      name: ""
    }
  },

  methods: {
    validate(e) {
      this.name = e.target.value = e.target.value
        .replace(REGEX_SPACE, "-")
        .replace(REGEX_BAD_CHARS, "");
    },

    // TODO: Need to respond to errors from the server
    createChannel() {
      this.$emit("createChannel", this.name);
    }
  }
};
</script>

<style scoped>

</style>