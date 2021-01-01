<template>
  <div class="user-list-item" :class="{'offline': offline}">
    <img
      class="user-picture"
      :src="userInfo.picture"
      alt="User picture"
      width="32"
      height="32"
      referrerpolicy="no-referrer"
    />
    <div class="user-name ellipsis-truncate">{{ userInfo.name }}</div>
  </div>
</template>

<script>
export default {
  name: "User",

  props: {
    offline: Boolean,
    userInfo: Object
  }
};
</script>

<style>

</style>
