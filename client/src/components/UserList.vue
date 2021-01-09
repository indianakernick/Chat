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
    //connected: Boolean
  },

  /*data() {
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
  },*/

  //watch: {
    //connected(online) {
    //  this.userStatusChanged(USER_ID, online ? "online" : "offline");
    //},

    /*userList: {
      // This doesn't work because users don't have status set on them
      // I don't really get this.
      // Why did I write it int he first place.
      handler(users) {
        this.onlineUserList = [];
        this.offlineUserList = [];
        for (const user of users) {
          if (user.status === "online") {
            this.onlineUserList.push(user.user_id);
          } else if (user.status === "offline") {
            this.offlineUserList.push(user.user_id);
          }
        }
      },
      deep: true
    }
  },*/

  computed: {
    onlineUserList() {
      const list = [];
      for (const user of this.userList) {
        if (user.status === "online") {
          list.push(user.user_id);
        }
      }
      return list;
    },

    offlineUserList() {
      const list = [];
      for (const user of this.userList) {
        if (user.status === "offline") {
          list.push(user.user_id);
        }
      }
      return list;
    }
  }

  /*methods: {
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
      let index = binarySearch(from, item => userId - item);
      if (index < from.length && from[index] === userId) {
        from.splice(index, 1);
      }
      index = binarySearch(to, item => userId - item);
      to.splice(index, 0, userId);
    },

    userStatusChanged(userId, status) {
      switch (status) {
        case "online":
          this.moveUser(this.offlineUserList, this.onlineUserList, userId);
          break;
        case "offline":
          this.moveUser(this.onlineUserList, this.offlineUserList, userId);
          break;
      }
    }
  }*/
};
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
