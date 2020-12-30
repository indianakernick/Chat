<template>
  <!-- https://stackoverflow.com/a/61042200/4093378 -->
  <img
    class="group-list-item"
    :class="{'active': groupId === currentGroupId}"
    @click="selectGroup"
    :src="picture"
    :title="name"
    alt=""
    width="64"
    height="64"
    referrerpolicy="no-referrer"
  />
</template>

<script>
export default {
  name: "Group",

  props: {
    groupId: Number,
    name: String,
    picture: String,
    currentGroupId: Number
  },

  emits: [
    "selectGroup"
  ],

  methods: {
    selectGroup() {
      this.$emit("selectGroup", this.groupId);
    }
  }
};
</script>

<style>

</style>
