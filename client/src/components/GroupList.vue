<template>
  <div class="group-list-container scrollable-container">
    <div class="group-list-block scrollable-block">
      <div class="group-list-item-wrapper" v-for="group in groupList">
        <img
          class="group-list-item"
          :class="{'active': group.group_id === currentGroupId}"
          @click="$emit('selectGroup', group.group_id)"
          :src="group.picture"
          :title="group.name"
          alt="Group picture"
          width="64"
          height="64"
          referrerpolicy="no-referrer"
        />
        <div class="group-list-item-back"></div>
      </div>
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
$image-size: 64px;

.group-list-container {
  background-color: $column-group-back;
  flex: 0 0 $image-size + 2 * $padding !important;
}

.group-list-block {
  display: flex;
  flex-direction: column;
}

.group-list-item:hover, .group-list-item.active {
  border-radius: $image-size / 4;
  + .group-list-item-back {
    border-radius: $image-size / 4 - 1;
  }
}

.group-list-create:hover {
  border-radius: $image-size / 4;
}

.group-list-item, .group-list-create, .group-list-item-back {
  border-radius: $image-size / 2;
  cursor: pointer;
  transition: border-radius 0.2s ease;
}

.group-list-item {
  position: relative;
  z-index: 1;
}

.group-list-item-back {
  background-color: $group-item-back;
  position: absolute;
  width: $image-size - 2;
  height: $image-size - 2;
  left: 1px;
  top: 1px;
  border-radius: $image-size / 2 - 1;
}

.group-list-item-wrapper {
  flex: 0 0 $image-size;
  position: relative;
  margin: $padding $padding 0 $padding;
}

.group-list-create {
  margin: $padding;
  width: $image-size;
  height: $image-size;
  background-color: $group-create-back;
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
