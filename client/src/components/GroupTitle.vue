<template>
  <div class="column-title">
    <div class="ellipsis-truncate">{{ currentGroupName }}</div>
    <div class="edit-button" ref="button">&nbsp;=</div>
  </div>
  <Popper
    class="dropdown"
    ref="dropdown"
    placement="bottom-end"
    distance="16"
  >
    <div class="dropdown-button" @click="$emit('createChannel')">Create channel</div>
    <div class="dropdown-button" @click="$emit('invite')">Invite to group</div>
    <div
      class="dropdown-button"
      @click="$emit('renameGroup', currentGroupName)"
    >Rename group</div>
    <div class="dropdown-button">Leave group</div>
    <div class="dropdown-button">Delete group</div>
  </Popper>
</template>

<script>
import Popper from "./Popper.vue";

export default {
  name: "GroupTitle",

  components: {
    Popper
  },

  props: {
    currentGroupName: String
  },

  emits: [
    "createChannel",
    "invite",
    "renameGroup"
  ],

  created() {
    this.$nextTick(() => {
      this.$refs.dropdown.initDropdownButton(this.$refs.button);
    });
  }
};
</script>

<style>

</style>
