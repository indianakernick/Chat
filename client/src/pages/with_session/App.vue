<template>
  <ProfileNav :name="name" :picture="picture"/>
  <MessageList/>
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
      name: "",
      picture: ""
    }
  },

  created() {
    const req = new XMLHttpRequest();

    req.onload = () => {
      this.name = req.response.name;
      this.picture = req.response.picture;
    };

    req.onerror = () => {
      console.error("Error connecting");
    };

    req.responseType = "json";
    req.open("GET", `/api/user/${user_id}`);
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
