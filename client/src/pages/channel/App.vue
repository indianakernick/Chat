<template>
  <ProfileNav :userInfo="userInfo"/>
  <MessageList :userInfo="userInfo"/>
</template>

<script>
import ProfileNav from "@/components/ProfileNav.vue";
import MessageList from "@/components/MessageList.vue";

export default {
  name: "App",

  components: {
    ProfileNav,
    MessageList
  },

  data() {
    return {
      userInfo: {
        name: "",
        picture: ""
      }
    }
  },

  created() {
    const req = new XMLHttpRequest();

    req.onload = () => {
      this.userInfo.name = req.response.name;
      this.userInfo.picture = req.response.picture;
    };

    req.onerror = () => {
      console.error("Error connecting");
    };

    req.responseType = "json";
    req.open("GET", `/api/user/${USER_ID}`);
    req.send();
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
