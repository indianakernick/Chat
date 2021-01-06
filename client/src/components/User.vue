<template>
  <div class="user-list-item" :class="{'offline': offline}">
    <img
      class="user-picture"
      :src="'/img/user/' + userInfo.user_id + '_32.png'"
      alt="User picture"
      width="32"
      height="32"
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
