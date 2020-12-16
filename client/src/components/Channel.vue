<template>
  <div class="channel" @click="clicked">{{ name }}</div>
</template>

<script>
export default {
  name: "Channel",

  props: {
    channelId: Number,
    name: String
  },

  emits: [
    "selectChannel"
  ],

  methods: {
    clicked() {
      this.$emit("selectChannel", this.channelId);
    }
  }
};
</script>

<style scoped>

</style>