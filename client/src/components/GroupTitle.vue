<template>
  <div class="group-title">
    <!-- https://stackoverflow.com/a/61042200/4093378 -->
    <img :src="info.picture" alt="" width="64" height="64" referrerpolicy="no-referrer"/>
    <span>{{info.name}}</span>
  </div>
</template>

<script>
export default {
  name: "GroupTitle",

  data() {
    return {
      info: GROUP_INFO
    };
  }
};
</script>

<style scoped>

</style>