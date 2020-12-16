<template>
  <div class="channel-list">
    <Channel
      v-for="channel in channelList"
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

  props: {
    channelList: Object
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
