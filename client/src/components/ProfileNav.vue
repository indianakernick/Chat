<template>
  <!-- https://stackoverflow.com/a/61042200/4093378 -->
  <img :src="userInfo.picture" alt="" width="32" height="32" referrerpolicy="no-referrer"/>
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
