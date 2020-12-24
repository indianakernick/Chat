<template>
  <li
    class="list-group-item list-group-item-action"
    :class="{'active': channelId === currentChannelId}"
    @click="selectChannel"
  >
    {{ name }}
    <button @click.stop="deleteChannel" type="button" class="close">
      <span>&times;</span>
    </button>
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
    "selectChannel",
    "deleteChannel"
  ],

  methods: {
    selectChannel() {
      this.$emit("selectChannel", this.channelId);
    },

    deleteChannel() {
      this.$emit("deleteChannel", this.channelId, this.name);
    }
  }
};
</script>

<style>

</style>
