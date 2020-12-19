<template>
  <GroupTitle :groupInfo="groupInfo"/>
  <ProfileNav :userInfo="userInfo"/>
  <div class="row">
    <ChannelList
      class="col-4"
      @selectChannel="selectChannel"
      @createChannel="createChannel"
      :channelList="channelList"
      :currentChannelId="currentChannelId"
    />
    <div class="col-8">
      <div class="message-list-container">
        <MessageList
          v-for="channel in channelList"
          :key="channel.channel_id"
          v-show="currentChannelId === channel.channel_id"
          :ref="list => messageLists[channel.channel_id] = list"
          :userInfo="userInfo"
          :userInfoCache="userInfoCache"
        />
      </div>
      <MessageSender @sendMessage="sendMessage"/>
    </div>
  </div>
</template>

<script>
import GroupTitle from "@/components/GroupTitle.vue";
import ProfileNav from "@/components/ProfileNav.vue";
import ChannelList from "@/components/ChannelList.vue";
import MessageList from "@/components/MessageList.vue";
import MessageSender from "@/components/MessageSender.vue";

const INITIAL_RETRY_DELAY = 125;
const VISIBLE_MAX_RETRY_DELAY = 8000;
const HIDDEN_MAX_RETRY_DELAY = 32000;

import {DELETED_USER_INFO} from "@/components/Message";

const userInfoCache = {
  cache: {
    0: DELETED_USER_INFO
  },

  getUserInfo(userId) {
    if (!this.cache.hasOwnProperty(userId)) {
      this.cache[userId] = {
        name: "",
        picture: ""
      };

      const req = new XMLHttpRequest();

      req.onload = () => {
        this.cache[userId].name = req.response.name;
        this.cache[userId].picture = req.response.picture;
      };

      req.responseType = "json";
      req.open("GET", `/api/user/${userId}`);
      req.send();
    }

    return this.cache[userId];
  },
};

export default {
  name: "App",

  components: {
    GroupTitle,
    ProfileNav,
    ChannelList,
    MessageList,
    MessageSender
  },

  data() {
    userInfoCache.cache[USER_ID] = USER_INFO;
    return {
      groupInfo: GROUP_INFO,
      userInfo: USER_INFO,
      userInfoCache: userInfoCache,
      currentChannelId: CHANNEL_ID,
      channelList: CHANNEL_LIST,
      messageLists: {},
      retryDelay: INITIAL_RETRY_DELAY,
      connected: false,
      startTime: 0,
      currPingCount: 0,
      pingCount: 0
    }
  },

  created() {
    this.openConnection();
    // TODO: Don't forget to remove this
    window.app = this;
  },

  methods: {
    selectChannel(channelId) {
      this.currentChannelId = channelId;
      window.history.replaceState(null, "", `/channel/${GROUP_ID}/${channelId}`);
      const channelName = this.channelList.find(channel =>
        channel.channel_id === channelId
      ).name;
      document.title = this.groupInfo.name + "#" + channelName;
    },

    sendMessage(content) {
      if (!this.connected) return;
      this.messageLists[this.currentChannelId].sendMessage(content);
      this.socket.send(JSON.stringify({
        type: "post message",
        content: content,
        channel_id: this.currentChannelId
      }));
    },

    createChannel(name) {
      if (!this.connected) return;
      this.socket.send(JSON.stringify({
        type: "create channel",
        name: name
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
      this.socket = new WebSocket(`wss://${window.location.host}/api/socket/${GROUP_ID}`);
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
        if (event.code !== 1000) {
          setTimeout(this.retryConnection, this.getRetryDelay());
        }
      };
    },

    requestRecentFromChannel(channelId) {
      this.socket.send(`{"type":"request recent messages","channel_id":${channelId}}`);
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
      this.socket.send('{"type":"request channels"}');
      // TODO: Also request the current channel so we can load it faster
      // Need to deal with the situation where the current channel has been deleted
    },

    checkCurrentChannelValid() {
      const foundChannel = this.channelList.find(channel =>
        channel.channel_id === this.currentChannelId
      );
      if (foundChannel === undefined) {
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
        this.requestChannels();
      };
    },

    openConnection() {
      this.initSocket();
      this.initListeners();

      this.socket.onopen = () => {
        this.connected = true;
        this.requestRecent();
      };
    },

    startPingPong(count) {
      this.pingCount = this.currPingCount = count;
      this.startTime = performance.now();
      this.socket.send("a");
    },

    receiveMessage(event) {
      if (event.data === "b") {
        const endTime = performance.now();
        this.currPingCount -= 1;
        if (this.currPingCount > 0) {
          this.socket.send("a");
        } else if (this.currPingCount === 0) {
          const total = endTime - this.startTime;
          console.log("Total duration:", total);
          console.log("Average duration:", total / this.pingCount);
        }
        return;
      }

      console.log(event.data);
      const message = JSON.parse(event.data);
      switch (message.type) {
        case "error":
          console.error("Server error:", message.message);
          this.status = "An error has occurred";
          this.socket.close(1000);
          break;

        case "recent message":
          this.messageLists[message.channel_id].recentMessage(message);
          break;

        case "message receipt":
          this.messageLists[message.channel_id].messageReceipt(message);
          break;

        case "recent message list":
          this.messageLists[message.channel_id].recentMessageList(message);
          break;

        case "channel created":
          this.channelList.push({
            channel_id: message.channel_id, name: message.name
          });
          this.$nextTick(() => this.messageLists[message.channel_id].createEmpty());
          break;

        case "channel list":
          this.channelList = message.channels;
          this.checkCurrentChannelValid();
          this.requestRecent();
          break;
      }
    }
  }
};
</script>

<style lang="scss">
#app {
  width: 100%;
  height: 100%;
}

.message-list-container {
  overflow-y: scroll;
  height: 500px; // TEMPORARY
}
</style>
