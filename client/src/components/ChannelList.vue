<template>
  <div class="channel-list">
    <Channel
      v-for="channel in channelList"
      :channelId="channel.channel_id"
      :name="channel.name"
      @selectChannel="selectChannel"
    />
    <MessageSender @sendMessage="createChannel"/>
  </div>
</template>

<script>
import Channel from "./Channel.vue";
import MessageSender from "./MessageSender.vue";

// TODO: Using the MessageSender is just a quick hack for testing

export default {
  name: "ChannelList",

  components: {
    Channel,
    MessageSender
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
