<template>
  <div class="channel-title">
    #&nbsp;{{ currentChannelName }}
  </div>
</template>

<script>
export default {
  name: "ChannelTitle",

  props: {
    currentChannelName: String
  }
};
</script>

<style lang="scss">
@import "../scss/colors";

/* similar to group title */
.channel-title {
  padding: 8px;
  background-color: $channel-title-back;
  border-bottom: 1px solid $channel-title-border;
  color: $channel-title-text;
  font-weight: 500;
  font-size: 1.2rem;
}
</style>
