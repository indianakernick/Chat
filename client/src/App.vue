<template>
  <ProfileNav v-if="loggedIn" :name="name" :picture="picture"/>
  <Login v-if="notLoggedIn"/>
  <MessageList v-if="loggedIn"/>
</template>

<script>
import Login from "./components/Login.vue";
import ProfileNav from "./components/ProfileNav.vue";
import MessageList from "./components/MessageList.vue";

const CONNECTING = 0;
const LOGGED_IN = 1;
const NOT_LOGGED_IN = 2;

export default {
  name: "App",

  components: {
    Login,
    ProfileNav,
    MessageList
  },

  data() {
    return {
      state: CONNECTING,
      name: "",
      picture: ""
    }
  },

  computed: {
    loggedIn() {
      return this.state === LOGGED_IN;
    },

    notLoggedIn() {
      return this.state === NOT_LOGGED_IN;
    }
  },

  created() {
    const req = new XMLHttpRequest();

    req.onload = () => {
      if (Object.keys(req.response).length === 0) {
        this.state = NOT_LOGGED_IN;
      } else {
        this.state = LOGGED_IN;
        this.name = req.response.name;
        this.picture = req.response.picture;
      }
    };

    req.onerror = () => {
      console.error("Error connecting");
    };

    req.responseType = "json";
    req.open("GET", "/api/me");
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
