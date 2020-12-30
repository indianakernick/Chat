<template>
  <div class="channel-list-container scrollable-container">
    <div class="scrollable-block">
      <!-- Perhaps remove the Channel component and just put everything here -->
      <Channel
        v-for="channel in channelList"
        :channelId="channel.channel_id"
        :name="channel.name"
        :currentChannelId="currentChannelId"
        :connected="connected"
        @selectChannel="selectChannel"
        @deleteChannel="deleteChannel"
      />
    </div>
  </div>
</template>

<script>
import Channel from "./Channel.vue";

export default {
  name: "ChannelList",

  components: {
    Channel
  },

  props: {
    channelList: Array,
    currentChannelId: Number,
    connected: Boolean
  },

  emits: [
    "selectChannel",
    "deleteChannel"
  ],

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
/*
TODO: Use SCSS to define the color scheme in one place.
Give them descriptive names like "borderColor" or "timeColor"

black      #000000
dimgray    #696969
gray       #808080
darkgray   #a9a9a9
silver     #c0c0c0
lightgray  #d3d3d3
gainsboro  #dcdcdc
white      #ffffff
*/

.channel-list-container {
  background-color: gray;
}

.channel-list-item {
  color: lightgray;
  margin: 8px;
  padding: 4px 8px 4px 8px;
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  justify-content: space-between;
}

.channel-name {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.channel-name span {
  color: lightgray;
}

.channel-list-item:hover, .channel-list-item.active {
  background-color: darkgray;
  color: white;
}

.channel-list-item:hover .channel-edit, .active .channel-edit {
  visibility: visible;
}

.channel-edit {
  float: right;
  visibility: hidden;
  color: lightgray;
}

.channel-edit:hover {
  color: white;
}
</style>
