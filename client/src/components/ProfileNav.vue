<template>
  <img :src="userInfo.picture" alt="" width="32" height="32"/>
  <span>{{ userInfo.name }}</span>
</template>

<script>
export default {
  name: "ProfileNav",

  props: {
    userInfo: Object
  }
};
</script>

<style scoped>

</style>
