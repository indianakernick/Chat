<template>
  <div class="column-title">
    <div class="ellipsis-truncate">{{ userInfo.name }}</div>
    <div class="edit-button" ref="button">&nbsp;=</div>
  </div>
  <Popper
    class="dropdown"
    ref="dropdown"
    placement="bottom-end"
    distance="16"
  >
    <div class="dropdown-button" @click="$emit('renameUser', userInfo.name, userInfo.picture)">Change name</div>
    <div class="dropdown-button" @click="logout">Logout</div>
    <div class="dropdown-button" @click="$emit('deleteUser')">Delete account</div>
  </Popper>
</template>

<script>
import Popper from "./Popper.vue";

export default {
  name: "UserTitle",

  components: {
    Popper
  },

  emits: [
    "renameUser",
    "deleteUser"
  ],

  props: {
    userInfo: Object
  },

  created() {
    this.$nextTick(() => {
      this.$refs.dropdown.initDropdownButton(this.$refs.button);
    });
  },

  methods: {
    logout() {
      window.location.href = "/logout";
      window.navigating = true;
    }
  }
};
</script>

<style>

</style>
