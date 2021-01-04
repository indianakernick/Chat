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
          :ref="button => button ? buttons[channel.channel_id] = button : delete buttons[channel.channel_id]"
        >&nbsp;=</div>
      </div>
    </div>
  </div>
  <Popper
    v-for="channel in channelList"
    class="dropdown"
    :ref="dropdown => dropdown ? dropdowns[channel.channel_id] = dropdown : delete dropdowns[channel.channel_id]"
    placement="bottom-end"
    distance="8"
    skid="8"
    arrowPadding="8"
  >
    <div class="dropdown-button">Rename channel</div>
    <div
      class="dropdown-button"
      @click="$emit('deleteChannel', channel.channel_id, channel.name)"
    >Delete channel</div>
  </Popper>
</template>

<script>
import Popper from "./Popper.vue";

export default {
  name: "ChannelList",

  components: {
    Popper
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

  data() {
    return {
      dropdowns: {},
      buttons: {}
    };
  },

  created() {
    this.initDropdowns();
  },

  watch: {
    channelList() {
      this.initDropdowns();
    }
  },

  methods: {
    initDropdowns() {
      this.$nextTick(() => {
        for (const channel of this.channelList) {
          this.dropdowns[channel.channel_id].initDropdownButton(this.buttons[channel.channel_id]);
        }
      });
    }
  }
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

.channel-list-item:last-child {
  margin-bottom: 8px;
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

.channel-list-item .edit-button[data-active] {
  visibility: visible;
}
</style>
