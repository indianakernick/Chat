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
    <div class="dropdown-button">Change name</div>
    <div class="dropdown-button">Logout</div>
    <div class="dropdown-button">Delete account</div>
  </Popper>
</template>

<script>
import Popper from "./Popper.vue";

export default {
  name: "UserTitle",

  components: {
    Popper
  },

  props: {
    userInfo: Object
  },

  created() {
    this.$nextTick(() => {
      this.$refs.dropdown.initDropdownButton(this.$refs.button);
    });
  }
};
</script>

<style>

</style>
