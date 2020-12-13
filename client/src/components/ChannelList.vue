<template>
  <div class="channel-list">
    <Channel
        v-for="channel in channels"
        :channelId="channel.channel_id"
        :name="channel.name"
    ></Channel>
  </div>
</template>

<script>
import Channel from "./Channel.vue";

export default {
  name: "ChannelList",

  components: {
    Channel
  },

  data() {
    return {
      channels: CHANNEL_LIST
    };
  }
};
</script>

<style scoped>

</style>