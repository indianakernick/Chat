<template>
  <GroupTitle/>
  <ProfileNav :userInfo="userInfo"/>
  <ChannelList/>
  <MessageList :userInfo="userInfo"/>
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
      userInfo: USER_INFO
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
