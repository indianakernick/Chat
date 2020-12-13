<template>
  <div class="channel-list">
    <Channel
        v-for="channel in channels"
        :channelId="channel.channelId"
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
      channels: []
    }
  },

  created() {
    const req = new XMLHttpRequest();
    req.onload = () => {
      this.channels = req.response.map(channel => {
        return {
          channelId: channel.channel_id,
          name: channel.name
        };
      })
    };
    req.responseType = "json";
    req.open("GET", `/api/group/${GROUP_ID}/channels`);
    req.send();
  }
};
</script>

<style scoped>

</style>