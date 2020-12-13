<template>
  <span class="channel">{{name}}</span>
</template>

<script>
export default {
  name: "Channel",

  props: {
    channelId: Number,
    name: String
  }
};
</script>

<style scoped>

</style>