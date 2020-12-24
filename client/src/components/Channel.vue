<template>
  <li
    class="list-group-item list-group-item-action"
    :class="{'active': channelId === currentChannelId}"
    @click="clicked"
  >{{ name }}</li>
</template>

<script>
export default {
  name: "Channel",

  props: {
    channelId: Number,
    name: String,
    currentChannelId: Number
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

<style>
/*.channel {
  display: flex !important;
  justify-content: space-between;
  align-items: center;
}*/
</style>
