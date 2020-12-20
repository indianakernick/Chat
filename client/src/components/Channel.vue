<template>
  <li
    class="d-flex justify-content-between align-items-center list-group-item list-group-item-action"
    :class="{'active': channelId === currentChannelId}"
    @click="clicked"
  >
    {{ name }}
    <span class="badge badge-secondary badge-pill">1</span>
  </li>
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
