<template>
  <GroupTitle/>
  <ProfileNav :userInfo="userInfo"/>
  <ChannelList @channelSelected="channelSelected"/>
  <MessageList :userInfo="userInfo" :channelId="channelId"/>
</template>

<script>
import GroupTitle from "@/components/GroupTitle.vue";
import ProfileNav from "@/components/ProfileNav.vue";
import ChannelList from "@/components/ChannelList.vue";
import MessageList from "@/components/MessageList.vue";

export default {
  name: "App",

  components: {
    GroupTitle,
    ProfileNav,
    ChannelList,
    MessageList
  },

  data() {
    return {
      userInfo: USER_INFO,
      channelId: CHANNEL_ID,
      channelNames: CHANNEL_LIST.reduce((names, info) => {
        names[info.channel_id] = info.name;
        return names;
      }, {})
    }
  },

  methods: {
    channelSelected(channelId) {
      console.assert(this.channelNames.hasOwnProperty(channelId));
      this.channelId = channelId;
      window.history.replaceState(null, "", `/channel/${GROUP_ID}/${channelId}`);
      document.title = GROUP_INFO.name + "#" + this.channelNames[channelId];
    }
  }
};
</script>

<style>
html, body {
  margin: 0;
  width: 100%;
  height: 100%;
}

#app {
  width: 100%;
  height: 100%;
}
</style>
