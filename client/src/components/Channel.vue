<template>
  <div class="channel" @click="clicked">{{name}}</div>
</template>

<script>
export default {
  name: "Channel",

  props: {
    channelId: Number,
    name: String
  },

  emits: [
    "channelSelected"
  ],

  methods: {
    clicked() {
      this.$emit("channelSelected", this.channelId);
    }
  }
};
</script>

<style scoped>

</style>