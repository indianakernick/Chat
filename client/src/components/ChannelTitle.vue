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

<style>
/* similar to group title */
.channel-title {
  padding: 8px;
  background-color: dimgray;
  border-bottom: 1px solid black;
  color: white;
  font-weight: 500;
  font-size: 1.2rem;
}
</style>
