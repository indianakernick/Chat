<template>
  <div class="group-list-container scrollable-container">
    <div class="group-list-block scrollable-block">
      <img
        v-for="group in groupList"
        class="group-list-item"
        :class="{'active': group.group_id === currentGroupId}"
        @click="$emit('selectGroup', group.group_id)"
        :src="group.picture64"
        alt="Group picture"
        width="64"
        height="64"
        :ref="img => img ? groupImages[group.group_id] = img : delete groupImages[group.group_id]"
      />
      <div
        class="group-list-create"
        @click="$emit('createGroup')"
        ref="createButton"
      ><span>+</span></div>
    </div>
  </div>
  <Popper
    v-for="group in groupList"
    class="tooltip"
    placement="right"
    distance="8"
    :ref="tooltip => tooltip ? groupTooltips[group.group_id] = tooltip : delete groupTooltips[group.group_id]"
  >{{ group.name }}</Popper>
  <Popper
    class="tooltip"
    placement="right"
    distance="8"
    ref="createTooltip"
  >Create group</Popper>
</template>

<script>
import Popper from "./Popper.vue";

export default {
  name: "GroupList",

  components: {
    Popper
  },

  props: {
    groupList: Array,
    currentGroupId: Number
  },

  emits: [
    "selectGroup",
    "createGroup"
  ],

  data() {
    return {
      groupImages: {},
      groupTooltips: {}
    }
  },

  created() {
    this.initTooltips();
  },

  watch: {
    groupList: {
      handler() {
        this.initTooltips();
      },
      deep: true
    }
  },

  methods: {
    initTooltips() {
      this.$nextTick(() => {
        for (const group of this.groupList) {
          this.groupTooltips[group.group_id].initTooltipButton(this.groupImages[group.group_id]);
        }
        this.$refs.createTooltip.initTooltipButton(this.$refs.createButton);
      });
    }
  }
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

.group-list-create:hover, .group-list-item:hover, .group-list-item.active {
  border-radius: 25%;
}

.group-list-create, .group-list-item {
  border-radius: 50%;
  cursor: pointer;
  transition: border-radius 0.2s ease;
}

.group-list-item {
  flex: 0 0 $image-size;
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
