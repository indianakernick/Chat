<template>
  <div class="column-title">
    <div class="ellipsis-truncate">{{ currentGroupName }}</div>
    <div class="edit-button" ref="button">&nbsp;=</div>
  </div>
  <Popper
    class="dropdown"
    ref="dropdown"
    placement="bottom-end"
    offset=16
    style="width: calc(100% - 16px)"
  >
    <div class="dropdown-button" @click="$emit('createChannel')">Create channel</div>
    <div class="dropdown-button" @click="$emit('invitePeople')">Invite people</div>
    <div class="dropdown-button">Rename group</div>
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
    "invitePeople"
  ],

  created() {
    this.$nextTick(() => {
      this.initPopper(this.$refs.button, this.$refs.dropdown);
    });
  },

  methods: {
    initPopper(button, dropdown) {
      button.onclick = () => {
        if (dropdown.toggle(button)) {
          button.setAttribute("data-active", "");
        } else {
          button.removeAttribute("data-active");
        }
      };
    }
  }
};
</script>

<style lang="scss">
@import "../scss/colors";

.dropdown-button {
  padding: 4px 8px;
  border-radius: 4px;
  margin: 8px 8px 0 8px;
  cursor: pointer;
}

.dropdown-button:last-child {
  margin-bottom: 8px;
}

.dropdown-button:hover {
  background-color: $gray-800;
}
</style>
