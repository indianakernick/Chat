<template>
  <div class="channel">{{name}}</div>
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