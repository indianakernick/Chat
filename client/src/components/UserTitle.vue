<template>
  <div class="column-title">
    <div class="ellipsis-truncate">{{ userInfo.name }}</div>
    <div class="edit-button">&nbsp;=</div>
  </div>
</template>

<script>
export default {
  name: "UserTitle",

  props: {
    userInfo: Object
  }
};
</script>

<style>

</style>
c