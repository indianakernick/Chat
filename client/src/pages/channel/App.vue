<template>
  <template v-if="groupList.length > 0">
    <GroupList
      :groupList="groupList"
      :currentGroupId="currentGroupId"
      @selectGroup="selectGroup"
      @createGroup="showCreateGroupDialog"
    />

    <div class="channel-column narrow-column">
      <GroupTitle
        :currentGroupId="currentGroupId"
        :currentGroupName="currentGroupName"
        :currentGroupPicture="currentGroupPicture"
        @createChannel="showCreateChannelDialog"
        @invite="showInviteDialog"
        @renameGroup="showRenameGroupDialog"
        @deleteGroup="showDeleteGroupDialog"
      />
      <ChannelList
        :channelList="channelList"
        :currentChannelId="currentChannelId"
        :connected="connected"
        @selectChannel="selectChannel"
        @renameChannel="showRenameChannelDialog"
        @deleteChannel="showDeleteChannelDialog"
      />
    </div>

    <div class="message-column">
      <ChannelTitle :currentChannelName="currentChannelName"/>
      <div class="message-list-container scrollable-container">
        <MessageList
          class="scrollable-block"
          v-for="channel in channelList"
          :key="channel.channel_id"
          v-show="currentChannelId === channel.channel_id"
          :shown="currentChannelId === channel.channel_id"
          :ref="list => list ? messageLists[channel.channel_id] = list : delete messageLists[channel.channel_id]"
          :userInfo="userInfo"
          :userInfoCache="userInfoCache"
        />
      </div>
      <MessageSender
        :connected="connected"
        :currentChannelName="currentChannelName"
        @sendMessage="sendMessage"
      />
    </div>

    <div class="user-column narrow-column">
      <UserTitle
        :userInfo="userInfo"
        @renameUser="showRenameUserDialog"
        @deleteUser="showDeleteUserDialog"
      />
      <UserList
        :userList="userList"
        :userInfoCache="userInfoCache"
        ref="userList"
      />
    </div>
  </template>

  <template v-else>
    <NoGroupsDialog @createGroup="showCreateGroupDialog"/>
  </template>

  <ChannelCreateOrRenameDialog
    @createChannel="createChannel"
    @renameChannel="renameChannel"
    ref="createOrRenameChannelDialog"
  />
  <ChannelDeleteDialog @deleteChannel="deleteChannel" ref="deleteChannelDialog"/>
  <GroupCreateOrRenameDialog
    @createGroup="createGroup"
    @renameGroup="renameGroup"
    ref="createOrRenameGroupDialog"
  />
  <InviteDialog :groupId="currentGroupId" :groupName="currentGroupName" ref="inviteDialog"/>
  <UserRenameDialog ref="renameUserDialog"/>
  <GroupDeleteDialog ref="deleteGroupDialog"/>
  <UserDeleteDialog ref="deleteUserDialog"/>
</template>

<script>
import ChannelTitle from "@/components/ChannelTitle.vue";
import GroupTitle from "@/components/GroupTitle.vue";
import GroupList from "@/components/GroupList.vue";
import ChannelList from "@/components/ChannelList.vue";
import UserTitle from "@/components/UserTitle.vue";
import UserList from "@/components/UserList.vue";
import MessageList from "@/components/MessageList.vue";
import InviteDialog from "@/components/InviteDialog.vue";
import MessageSender from "@/components/MessageSender.vue";
import GroupCreateOrRenameDialog from "@/components/GroupCreateOrRenameDialog.vue";
import ChannelCreateOrRenameDialog from "@/components/ChannelCreateOrRenameDialog.vue";
import ChannelDeleteDialog from "@/components/ChannelDeleteDialog.vue";
import NoGroupsDialog from "@/components/NoGroupsDialog.vue";
import UserRenameDialog from "@/components/UserRenameDialog.vue";
import GroupDeleteDialog from "@/components/GroupDeleteDialog.vue";
import UserDeleteDialog from "@/components/UserDeleteDialog.vue";
import userInfoCache from "@/assets/js/userInfoCache.js";
import { comp64 } from "@/assets/js/ImageCompositor";
import { reactive, watchEffect } from "vue";
import { binarySearchFind, binarySearchInsert } from "@/assets/js/binarySearch.js";

const INITIAL_RETRY_DELAY = 125;
const VISIBLE_MAX_RETRY_DELAY = 8000;
const HIDDEN_MAX_RETRY_DELAY = 32000;

export default {
  name: "App",

  components: {
    ChannelTitle,
    GroupTitle,
    ChannelList,
    GroupList,
    UserTitle,
    UserList,
    MessageList,
    InviteDialog,
    MessageSender,
    GroupCreateOrRenameDialog,
    ChannelCreateOrRenameDialog,
    ChannelDeleteDialog,
    NoGroupsDialog,
    UserRenameDialog,
    GroupDeleteDialog,
    UserDeleteDialog
  },

  data() {
    const userList = [];
    for (const user of USER_LIST) {
      userInfoCache.setUserInfo(user.user_id, user.name, user.picture);
      const status = user.user_id === USER_ID ? "online" : "offline";
      userList.push({user_id: user.user_id, status: status});
    }

    const groupList = GROUP_LIST.map(this.initializeReactiveGroup);

    return {
      currentGroupId: GROUP_ID,
      currentChannelId: CHANNEL_ID,
      userInfo: userInfoCache.cache[USER_ID],
      userInfoCache: userInfoCache,
      userList: userList,
      groupList: groupList,
      channelList: CHANNEL_LIST,
      messageLists: {},
      retryDelay: INITIAL_RETRY_DELAY,
      connected: false
    }
  },

  created() {
    if (Notification.permission === "default") {
      Notification.requestPermission();
    }

    watchEffect(() => {
      const group = this.currentGroupName;
      const channel = this.currentChannelName;
      if (group.length && channel.length) {
        document.title = group + "#" + channel;
      }
    });

    if (this.groupList.length > 0) {
      this.openConnection();
    }
    // TODO: Don't forget to remove this
    window.app = this;
  },

  computed: {
    // should probably get the group name and picture as one object
    currentGroupName() {
      if (this.groupList.length > 0) {
        return this.groupList.find(group =>
          group.group_id === this.currentGroupId
        ).name;
      } else {
        return "";
      }
    },

    currentGroupPicture() {
      if (this.groupList.length > 0) {
        return this.groupList.find(group =>
          group.group_id === this.currentGroupId
        ).picture;
      } else {
        return "";
      }
    },

    currentChannelName() {
      if (this.channelList.length > 0) {
        return this.channelList.find(channel =>
          channel.channel_id === this.currentChannelId
        ).name;
      } else {
        return "";
      }
    }
  },

  watch: {
    connected(online) {
      const index = binarySearchFind(this.userList, item => USER_ID - item.user_id);
      this.userList[index].status = online ? "online" : "offline";
    }
  },

  methods: {
    initializeReactiveGroup(group) {
      const reactiveGroup = reactive({
        group_id: group.group_id,
        name: group.name,
        picture: group.picture,
        picture64: ""
      });
      watchEffect(() => {
        comp64.composite(reactiveGroup.picture, url => {
          reactiveGroup.picture64 = url;
        });
      });
      return reactiveGroup;
    },

    showCreateChannelDialog() {
      if (!this.connected) return;
      this.$refs.createOrRenameChannelDialog.showCreate();
    },

    showRenameChannelDialog(channelId, name) {
      if (!this.connected) return;
      this.$refs.createOrRenameChannelDialog.showRename(channelId, name);
    },

    showDeleteChannelDialog(channelId, name) {
      if (!this.connected) return;
      this.$refs.deleteChannelDialog.show(channelId, name);
    },

    showCreateGroupDialog() {
      if (!this.connected) return;
      this.$refs.createOrRenameGroupDialog.showCreate();
    },

    showRenameGroupDialog(name, picture) {
      if (!this.connected) return;
      this.$refs.createOrRenameGroupDialog.showRename(name, picture);
    },

    showDeleteGroupDialog(groupId, name) {
      if (!this.connected) return;
      this.$refs.deleteGroupDialog.show(groupId, name);
    },

    showInviteDialog() {
      if (!this.connected) return;
      this.$refs.inviteDialog.show();
    },

    showRenameUserDialog(name, picture) {
      if (!this.connected) return;
      this.$refs.renameUserDialog.show(name, picture);
    },

    showDeleteUserDialog() {
      if (!this.connected) return;
      this.$refs.deleteUserDialog.show();
    },

    selectChannel(channelId) {
      if (this.currentChannelId === channelId) return;
      this.currentChannelId = channelId;
      window.history.replaceState(null, "", `/channel/${this.currentGroupId}/${channelId}`);
    },

    selectGroup(groupId) {
      if (!this.connected) return;
      if (this.currentGroupId === groupId) return;
      this.socket.close(1000);
      this.currentGroupId = groupId;
      // This will connect to the socket, fetch the channel list, update the
      // current channel, then fetch the message lists.
      this.retryConnection();
    },

    createGroup(group) {
      if (!this.connected) return;
      this.socket.close(1000);
      this.groupList.push(this.initializeReactiveGroup(group));
      this.currentGroupId = group.group_id;
      this.retryConnection();
    },

    sendMessage(content) {
      if (!this.connected) return;
      this.messageLists[this.currentChannelId].sendMessage(content);
      this.socket.send(JSON.stringify({
        type: "create_message",
        content: content,
        channel_id: this.currentChannelId
      }));
    },

    createChannel(name) {
      if (!this.connected) return;
      this.socket.send(JSON.stringify({
        type: "create_channel",
        name: name
      }));
    },

    renameChannel(channelId, name) {
      if (!this.connected) return;
      this.socket.send(JSON.stringify({
        type: "rename_channel",
        channel_id: channelId,
        name: name
      }));
    },

    deleteChannel(channelId) {
      if (!this.connected) return;
      this.socket.send(JSON.stringify({
        type: "delete_channel",
        channel_id: channelId
      }));
    },

    renameGroup(name, picture) {
      if (!this.connected) return;
      this.socket.send(JSON.stringify({
        type: "rename_group",
        name: name,
        picture: picture
      }));
    },

    getRetryDelay() {
      const maximum = document.visibilityState === "hidden"
        ? HIDDEN_MAX_RETRY_DELAY
        : VISIBLE_MAX_RETRY_DELAY;
      const delay = this.retryDelay;
      this.retryDelay = Math.min(maximum, 2 * this.retryDelay);
      return delay;
    },

    resetRetryDelay() {
      this.retryDelay = INITIAL_RETRY_DELAY;
    },

    initSocket() {
      this.socket = new WebSocket(`wss://${window.location.host}/api/socket/${this.currentGroupId}`);
    },

    initListeners() {
      this.socket.onmessage = this.receiveMessage;

      this.socket.onerror = () => {
        this.connected = false;
      };

      this.socket.onclose = event => {
        this.connected = false;
        // 1000 means "normal closure"
        // https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent
        if (event.code === 1000) return;
        if (event.code === 4000) {
          if (!window.navigating) window.location.reload(true);
          return;
        }
        setTimeout(this.retryConnection, this.getRetryDelay());
      };
    },

    requestRecentFromChannel(channelId) {
      this.socket.send(`{"type":"request_recent_messages","channel_id":${channelId}}`);
    },

    requestRecent() {
      this.requestRecentFromChannel(this.currentChannelId);
      for (const channel of this.channelList) {
        if (channel.channel_id !== this.currentChannelId) {
          this.requestRecentFromChannel(channel.channel_id);
        }
      }
    },

    requestChannels() {
      this.socket.send('{"type":"request_channels"}');
    },

    requestUsers() {
      this.socket.send('{"type":"request_users"}');
    },

    checkCurrentChannelValid() {
      const has = this.channelList.some(channel =>
        channel.channel_id === this.currentChannelId
      );
      if (!has) {
        this.selectChannel(this.channelList[0].channel_id);
      }
    },

    retryConnection() {
      this.initSocket();

      this.socket.onerror = () => {
        setTimeout(this.retryConnection, this.getRetryDelay());
      };

      this.socket.onopen = () => {
        this.connected = true;
        this.resetRetryDelay();
        this.initListeners();
        // TODO: I think we also need to request the group list
        this.requestChannels();
        this.requestUsers();
      };
    },

    openConnection() {
      this.initSocket();
      this.initListeners();

      this.socket.onopen = () => {
        this.connected = true;
        this.requestRecent();
        this.requestUsers();
      };
    },

    handleError(category, code) {
      switch (category) {
        case "application":
        case "request":
          console.error("Server error:", code);
          // TODO: This status message isn't shown
          this.status = "An error has occurred";
          this.socket.close(1000);
          break;

        case "channel_create":
          this.$refs.createOrRenameChannelDialog.error();
          break;

        case "channel_rename":
          this.$refs.createOrRenameChannelDialog.error();
          break;

        case "channel_delete":
          this.$refs.deleteChannelDialog.error();
          break;

        case "group_rename":
          this.$refs.createOrRenameGroupDialog.error(code);
          break;
      }
    },

    receiveMessage(event) {
      const message = JSON.parse(event.data);
      console.log(message);
      switch (message.type) {
        case "error":
          this.handleError(message.category, message.code);
          break;

        case "recent_message":
          this.messageLists[message.channel_id].recentMessage(message);
          break;

        case "message_receipt":
          this.messageLists[message.channel_id].messageReceipt(message);
          break;

        case "recent_message_list":
          this.messageLists[message.channel_id].recentMessageList(message);
          break;

        case "channel_created":
          this.channelList.push({
            channel_id: message.channel_id, name: message.name
          });
          this.$nextTick(() => this.messageLists[message.channel_id].createEmpty());
          if (this.$refs.createOrRenameChannelDialog.channelCreated(message.name)) {
            this.selectChannel(message.channel_id);
          }
          break;

        case "channel_list":
          this.channelList = message.channels;
          this.checkCurrentChannelValid();
          this.requestRecent();
          break;

        case "channel_deleted": {
          const index = this.channelList.findIndex(channel =>
            channel.channel_id === message.channel_id
          );
          if (index !== -1) {
            this.channelList.splice(index, 1);
            this.checkCurrentChannelValid();
            this.$refs.deleteChannelDialog.channelDeleted(message.channel_id);
            this.$refs.createOrRenameChannelDialog.channelDeleted(message.channel_id);
          }
          break;
        }

        case "channel_renamed": {
          const index = this.channelList.findIndex(channel =>
            channel.channel_id === message.channel_id
          );
          if (index !== -1) {
            this.channelList[index].name = message.name;
            this.$refs.createOrRenameChannelDialog.channelRenamed(message.channel_id, message.name);
            this.$refs.deleteChannelDialog.channelRenamed(message.channel_id, message.name);
          }
          break;
        }

        case "user_list": {
          const userList = [];
          for (const user of message.users) {
            userInfoCache.setUserInfo(user.user_id, user.name, user.picture);
            userList.push({user_id: user.user_id, status: user.status});
          }
          this.userList = userList;
          break;
        }

        case "user_status_changed": {
          const index = binarySearchInsert(this.userList, item => message.user_id - item.user_id);
          if (index < this.userList.length && this.userList[index].user_id === message.user_id) {
            this.userList[index].status = message.status;
          } else if (message.status === "online") {
            this.userList.splice(index, 0, { user_id: message.user_id, status: message.status });
          }
          break;
        }

        case "user_renamed":
          this.userInfoCache.setUserInfo(message.user_id, message.name, message.picture);
          break;

        case "user_deleted":
          for (const channelId in this.messageLists) {
            this.messageLists[channelId].deleteUser(message.user_id);
          }
          const index = binarySearchFind(this.userList, item => message.user_id - item.user_id);
          if (index !== null) {
            this.userList.splice(index, 1);
          }
          this.userInfoCache.removeUserInfo(message.user_id);
          break;

        case "group_renamed": {
          const index = this.groupList.findIndex(group =>
            group.group_id === message.group_id
          );
          if (index !== -1) {
            this.groupList[index].name = message.name;
            this.groupList[index].picture = message.picture;
            if (message.group_id === this.currentGroupId) {
              this.$refs.createOrRenameGroupDialog.groupRenamed(message.name, message.picture);
              this.$refs.deleteGroupDialog.groupRenamed(message.name);
            }
          }
          break;
        }

        case "group_deleted": {
          const index = this.groupList.findIndex(group =>
            group.group_id === message.group_id
          );
          if (index !== -1) {
            this.groupList.splice(index, 1);
            if (message.group_id === this.currentGroupId) {
              this.$refs.createOrRenameGroupDialog.groupDeleted();
              this.$refs.deleteGroupDialog.groupDeleted();
            }
          }
        }
      }
    }
  }
};
</script>

<style lang="scss">
@import "../../scss/colors";
@import "../../scss/common";

html, body {
  margin: 0;
  height: 100%;
  overflow: hidden;
}

#app {
  width: 100vw;
  height: 100vh;
  display: flex;
}

.narrow-column {
  display: flex;
  flex-direction: column;
  flex: 0 0 calc(100% / 6);
  max-width: calc(100% / 6);
  min-width: 160px;
  position: relative;
}

.channel-column {
  background-color: $column-channel-back;
}

.message-column {
  background-color: $column-message-back;
  display: flex;
  flex-direction: column;
  flex-grow: 1;
}

.user-column {
  background-color: $column-user-back;
}

.message-list-container {
  display: flex;
  flex-direction: column-reverse;
  z-index: 1; /* Ensure that the focus outline is above everything else */
}
</style>
