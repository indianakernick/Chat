<template>
  <div class="group-list-container scrollable-container">
    <div class="scrollable-block">
      <img
        v-for="group in groupList"
        class="group-list-item"
        :class="{'active': group.group_id === currentGroupId}"
        @click="$emit('selectGroup', group.group_id)"
        :src="group.picture"
        :title="group.name"
        :alt="group.name"
        width="64"
        height="64"
        referrerpolicy="no-referrer"
      />
      <div
        class="group-list-create"
        @click="$emit('createGroup')"
        title="Create group"
      ><span>+</span></div>
    </div>
  </div>
</template>

<script>
export default {
  name: "GroupList",

  props: {
    groupList: Array,
    currentGroupId: Number
  },

  emits: [
    "selectGroup",
    "createGroup"
  ]
};
</script>

<style lang="scss">
@import "../scss/colors";

$padding: 8px;
$imageSize: 64px;

.group-list-container {
  background-color: $column-group-back;
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
  background-color: $group-item-back;
}

.group-list-create {
  margin: $padding;
  width: $imageSize;
  height: $imageSize;
  background-color: $group-item-back;
  display: flex;
  justify-content: center;
  align-items: center;
}

.group-list-create > span {
  font-size: 3.5rem;
  font-family: monospace;
  font-weight: 200;
  color: $group-create-text;
}
</style>
