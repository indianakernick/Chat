<template>
  <div class="column-title">
    <div class="ellipsis-truncate">{{ currentGroupName }}</div>
    <div class="edit-button">&nbsp;=</div>
  </div>
</template>

<script>
export default {
  name: "GroupTitle",

  props: {
    currentGroupName: String
  }
};
</script>

<style>

</style>
