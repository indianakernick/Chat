<template>
  <ul class="list-group">
    <Channel
      v-for="channel in channelList"
      :channelId="channel.channel_id"
      :name="channel.name"
      :currentChannelId="currentChannelId"
      @selectChannel="selectChannel"
    />
    <ChannelCreate @createChannel="createChannel"/>
  </ul>
</template>

<script>
import Channel from "./Channel.vue";
import ChannelCreate from "./ChannelCreate.vue";

export default {
  name: "ChannelList",

  components: {
    Channel,
    ChannelCreate
  },

  emits: [
    "selectChannel",
    "createChannel"
  ],

  props: {
    channelList: Object,
    currentChannelId: Number
  },

  methods: {
    selectChannel(channelId) {
      this.$emit("selectChannel", channelId);
    },

    createChannel(name) {
      this.$emit("createChannel", name);
    }
  }
};
</script>

<style>

</style>
