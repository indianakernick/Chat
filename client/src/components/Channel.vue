<template>
  <div
    class="channel-list-item"
    :class="{'active': channelId === currentChannelId}"
    @click="selectChannel"
  >
    <div class="channel-name"><span>#&nbsp;</span>{{ name }}</div>
    <div class="channel-edit" @click.stop="deleteChannel">&nbsp;=</div>
  </div>
</template>

<script>
export default {
  name: "Channel",

  props: {
    channelId: Number,
    name: String,
    currentChannelId: Number,
    connected: Boolean
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
