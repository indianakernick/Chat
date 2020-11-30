<template>
  <MessageList/>
</template>

<script>
import MessageList from "./components/MessageList.vue";

export default {
  name: "App",

  components: {
    MessageList
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
