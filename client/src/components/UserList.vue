<template>
  <div class="scrollable-container">
    <div class="scrollable-block">
      <div
        v-if="onlineUserList.length > 0"
        class="user-section-title"
      >Online ({{ onlineUserList.length }})</div>
      <User
        v-for="userId in onlineUserList"
        :userInfo="userInfoCache.getUserInfo(userId)"
        :offline="false"
      />

      <div
        v-if="offlineUserList.length > 0"
        class="user-section-title"
      >Offline ({{ offlineUserList.length }})</div>
      <User
        v-for="userId in offlineUserList"
        :userInfo="userInfoCache.getUserInfo(userId)"
        :offline="true"
      />
    </div>
  </div>
</template>

<script>
import User from "./User.vue";

export default {
  name: "UserList",

  components: {
    User
  },

  props: {
    userList: Array,
    userInfoCache: Object,
    connected: Boolean
  },

  watch: {
    connected(online) {
      if (online) {
        this.userOnline(USER_ID);
      } else {
        this.userOffline(USER_ID);
      }
    }
  },

  data() {
    let offline = [];
    for (const user of this.userList) {
      if (user.user_id !== USER_ID) {
        offline.push(user.user_id);
      }
    }
    return {
      onlineUserList: [USER_ID],
      offlineUserList: offline
    }
  },

  methods: {
    onlineUsers(users) {
      this.onlineUserList = [];
      this.offlineUserList = [];
      for (const user of this.userList) {
        if (users.indexOf(user.user_id) === -1) {
          this.offlineUserList.push(user.user_id);
        } else {
          this.onlineUserList.push(user.user_id);
        }
      }
    },

    moveUser(from, to, userId) {
      let index = from.indexOf(userId);
      if (index !== -1) {
        from.splice(index, 1);
        // TODO: binary search and insert
        to.push(userId);
        to.sort();
      }
    },

    userOnline(userId) {
      this.moveUser(this.offlineUserList, this.onlineUserList, userId);
    },

    userOffline(userId) {
      this.moveUser(this.onlineUserList, this.offlineUserList, userId);
    }
  }
}
</script>

<style lang="scss">
@import "../scss/colors";

.user-list-item {
  padding: 8px 8px 0 8px;
  display: flex;
  align-items: center;
}

.offline {
  opacity: 0.4;
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
