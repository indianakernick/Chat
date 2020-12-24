<template>
  <ul class="list-group">
    <Channel
      v-for="channel in channelList"
      :channelId="channel.channel_id"
      :name="channel.name"
      :currentChannelId="currentChannelId"
      @selectChannel="selectChannel"
      @deleteChannel="deleteChannel"
    />
  </ul>
</template>

<script>
// TODO: Does this component even need to exist?

import Channel from "./Channel.vue";

export default {
  name: "ChannelList",

  components: {
    Channel
  },

  emits: [
    "selectChannel",
    "deleteChannel"
  ],

  props: {
    channelList: Object,
    currentChannelId: Number
  },

  methods: {
    selectChannel(channelId) {
      this.$emit("selectChannel", channelId);
    },

    deleteChannel(channelId, name) {
      this.$emit("deleteChannel", channelId, name);
    }
  }
};
</script>

<style>

</style>
