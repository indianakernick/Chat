<template>
  <div class="group-title">
    <!-- https://stackoverflow.com/a/61042200/4093378 -->
    <img :src="groupInfo.picture" alt="" width="64" height="64" referrerpolicy="no-referrer"/>
    <span>{{ groupInfo.name }}</span>
  </div>
</template>

<script>
export default {
  name: "GroupTitle",

  props: {
    groupInfo: Object
  }
};
</script>

<style>

</style>
