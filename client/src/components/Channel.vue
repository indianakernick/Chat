<template>
  <li
    :class="{'list-group-item': true, 'list-group-item-action': true, 'active': channelId === currentChannelId}"
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

</style>
