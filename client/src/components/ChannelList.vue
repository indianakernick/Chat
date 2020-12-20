<template>
  <ul class="list-group">
    <Channel
      v-for="channel in channelList"
      :channelId="channel.channel_id"
      :name="channel.name"
      :currentChannelId="currentChannelId"
      @selectChannel="selectChannel"
    />
  </ul>
</template>

<script>
import Channel from "./Channel.vue";

export default {
  name: "ChannelList",

  components: {
    Channel
  },

  emits: [
    "selectChannel"
  ],

  props: {
    channelList: Object,
    currentChannelId: Number
  },

  methods: {
    selectChannel(channelId) {
      this.$emit("selectChannel", channelId);
    }
  }
};
</script>

<style>

</style>
