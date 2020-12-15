<template>
  <div class="channel-list">
    <Channel
      v-for="channel in channels"
      :channelId="channel.channel_id"
      :name="channel.name"
      @channelSelected="channelSelected"
    />
  </div>
</template>

<script>
import Channel from "./Channel.vue";

export default {
  name: "ChannelList",

  components: {
    Channel
  },

  emits: [
    "channelSelected"
  ],

  data() {
    return {
      channels: CHANNEL_LIST
    };
  },

  methods: {
    channelSelected(channelId) {
      this.$emit("channelSelected", channelId);
    }
  }
};
</script>

<style scoped>

</style>
