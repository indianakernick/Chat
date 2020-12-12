<template>
  <Login/>
</template>

<script>
import Login from "@/components/Login.vue";

export default {
  name: "App",

  components: {
    Login
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
