<template>
  <div class="group-list-container scrollable-container">
    <div class="scrollable-block">
      <!-- Perhaps remove the Group component and just put everything here -->
      <Group
        v-for="group in groupList"
        :groupId="group.group_id"
        :name="group.name"
        :picture="group.picture"
        :currentGroupId="currentGroupId"
        @selectGroup="selectGroup"
      />
      <div
        class="group-list-create"
        @click="createGroup"
        title="Create group"
      ><span>+</span></div>
    </div>
  </div>
</template>

<script>
import Group from "./Group.vue";

export default {
  name: "GroupList",

  components: {
    Group
  },

  props: {
    groupList: Array,
    currentGroupId: Number
  },

  emits: [
    "selectGroup",
    "createGroup"
  ],

  methods: {
    selectGroup(groupId) {
      this.$emit("selectGroup", groupId)
    },

    createGroup() {
      this.$emit("createGroup");
    }
  }
};
</script>

<style lang="scss">
$padding: 8px;
$imageSize: 64px;

.group-list-container {
  background-color: dimgray;
  flex: 0 0 $imageSize + 2 * $padding !important;
}

.group-list-create:hover, .group-list-item:hover, .group-list-item.active {
  border-radius: $imageSize / 4;
}

.group-list-item, .group-list-create {
  border-radius: $imageSize / 2;
  cursor: pointer;
  transition: border-radius 0.2s ease;
}

.group-list-item {
  margin: $padding $padding 0 $padding;
  background-color: lightgray;
}

.group-list-create {
  margin: $padding;
  width: $imageSize;
  height: $imageSize;
  background-color: lightgray;
  display: flex;
  justify-content: center;
  align-items: center;
}

.group-list-create > span {
  font-size: 3.5rem;
  font-family: monospace;
  font-weight: 200;
}
</style>
