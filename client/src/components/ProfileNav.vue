<template>
  <img :src="picture" alt="" width="32" height="32"/>
  <span>{{ name }}</span>
</template>

<script>
export default {
  name: "ProfileNav",

  props: {
    name: String,
    picture: String
  }
};
</script>

<style scoped>

</style>
