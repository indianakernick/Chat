<template>
  <div class="channel-list">
    <Channel
      v-for="channel in channelList"
      :channelId="channel.channel_id"
      :name="channel.name"
      @selectChannel="selectChannel"
    />
    <ChannelCreate @createChannel="createChannel"/>
  </div>
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
    channelList: Object
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

<style scoped>

</style>
