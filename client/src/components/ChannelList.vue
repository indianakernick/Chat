<template>
  <div class="scrollable-container">
    <div class="scrollable-block">
      <div
        v-for="channel in channelList"
        class="channel-list-item"
        :class="{'active': channel.channel_id === currentChannelId}"
        @click="$emit('selectChannel', channel.channel_id)"
      >
        <div class="ellipsis-truncate">
          <span>#&nbsp;</span>{{ channel.name }}
        </div>
        <div
          class="edit-button"
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

.channel-list-item {
  color: $channel-list-text;
  padding: 4px 8px;
  cursor: pointer;
  display: flex;
  justify-content: space-between;
  margin: 8px 8px 0 8px;
  border-radius: 4px;
}

.channel-list-item span {
  color: $channel-list-text;
}

.channel-list-item:hover, .channel-list-item.active {
  background-color: $channel-item-hover-back;
  color: $channel-item-hover-text;

  .edit-button {
    visibility: visible;
  }
}

.channel-list-item .edit-button {
  visibility: hidden;
}
</style>
