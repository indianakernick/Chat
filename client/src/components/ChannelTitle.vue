<template>
  <div class="column-title">
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

</style>
