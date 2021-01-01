<template>
  <div class="user-list-item" :class="{'offline': offline}">
    <!-- Maybe the user list pictures should be smaller than the message pictures...? -->
    <img
      class="user-picture"
      :src="userInfo.picture"
      alt="User picture"
      width="48"
      height="48"
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
