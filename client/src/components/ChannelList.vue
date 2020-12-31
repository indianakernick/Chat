<template>
  <div class="channel-list-container scrollable-container">
    <div class="scrollable-block">
      <div
        v-for="channel in channelList"
        class="channel-list-item"
        :class="{'active': channel.channel_id === currentChannelId}"
        @click="$emit('selectChannel', channel.channel_id)"
      >
        <div class="channel-name"><span>#&nbsp;</span>{{ channel.name }}</div>
        <div
          class="channel-edit"
          @click.stop="$emit('deleteChannel', channel.channel_id, channel.name)"
        >&nbsp;=</div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: "ChannelList",

  props: {
    channelList: Array,
    currentChannelId: Number,
    connected: Boolean
  },

  emits: [
    "selectChannel",
    "deleteChannel"
  ]
};
</script>

<style lang="scss">
@import "../scss/colors";

.channel-list-container {
  background-color: $channel-list-back;
}

.channel-list-item {
  color: $channel-list-text;
  padding: 4px 8px 4px 8px;
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
  color: $channel-list-text;
}

.channel-list-item:hover, .channel-list-item.active {
  background-color: $channel-item-hover-back;
  color: $channel-item-hover-text;
}

.channel-list-item:hover .channel-edit, .active .channel-edit {
  visibility: visible;
}

.channel-edit {
  float: right;
  visibility: hidden;
  color: $channel-edit-text;
}

.channel-edit:hover {
  color: $channel-edit-hover-text;
}
</style>
