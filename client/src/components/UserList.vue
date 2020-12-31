<template>
  <div class="user-list-container scrollable-container">
    <div class="scrollable-block">
      <div
        v-if="onlineUserList.length > 0"
        class="user-section-title"
      >Online ({{ onlineUserList.length }})</div>
      <div v-for="user in onlineUserList" class="user-list-item">
        <img :src="user.picture" alt="" width="32" height="32" referrerpolicy="no-referrer"/>
        <div>{{ user.name }}</div>
      </div>

      <div
        v-if="offlineUserList.length > 0"
        class="user-section-title"
      >Offline ({{ offlineUserList.length }})</div>
      <div v-for="user in offlineUserList" class="user-list-item offline">
        <img :src="user.picture" alt="" width="32" height="32" referrerpolicy="no-referrer"/>
        <div>{{ user.name }}</div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: "UserList",

  props: {
    userList: Array
  },

  data() {
    // TODO: Determine online/offline status
    return {
      onlineUserList: this.userList.slice(0, 2),
      offlineUserList: this.userList.slice(2)
    }
  }
}
</script>

<style lang="scss">
@import "../scss/colors";

.user-list-container {
  background-color: $user-list-back;
}

.user-list-item {
  padding: 8px 8px 0 8px;
  display: flex;
  align-items: center;
}

.offline {
  opacity: 0.4;
}

.user-list-item div {
  font-weight: 500;
  color: $user-item-text;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.user-list-item img {
  border-radius: 16px;
  margin-right: 8px;
}

.user-section-title {
  color: $user-section-text;
  padding: 16px 8px 0 8px;
  font-size: 0.8rem;
}

.user-section-title:first-of-type {
  padding-top: 8px;
}
</style>
