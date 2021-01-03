<template>
  <div class="group-list-container scrollable-container">
    <div class="group-list-block scrollable-block">
      <img
        v-for="group in groupList"
        class="group-list-item"
        :class="{'active': group.group_id === currentGroupId}"
        @click="$emit('selectGroup', group.group_id)"
        :src="group.picture"
        alt="Group picture"
        width="64"
        height="64"
        :aria-describedby="'group-tooltip-' + group.group_id"
        :ref="img => groupImages[group.group_id] = img"
      />
      <div
        class="group-list-create"
        @click="$emit('createGroup')"
        aria-describedby="group-tooltip-create"
        ref="createButton"
      ><span>+</span></div>
    </div>
  </div>
  <div
    v-for="group in groupList"
    class="group-tooltip"
    :id="'group-tooltip-' + group.group_id"
    role="tooltip"
    :ref="tooltip => groupTooltips[group.group_id] = tooltip"
  >{{ group.name }}<div class="tooltip-arrow" data-popper-arrow/>
  </div>
  <div
    class="group-tooltip"
    id="group-tooltip-create"
    role="tooltip"
    ref="createTooltip"
  >Create group<div class="tooltip-arrow" data-popper-arrow/>
  </div>
</template>

<script>
import { createPopper } from "@popperjs/core/lib/popper-lite";
import arrow from "@popperjs/core/lib/modifiers/arrow";
import offset from "@popperjs/core/lib/modifiers/offset";

export default {
  name: "GroupList",

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
      poppers: [],
      groupImages: {},
      groupTooltips: {}
    }
  },

  created() {
    this.$nextTick(() => {
      for (const group of this.groupList) {
        const image = this.groupImages[group.group_id];
        const tooltip = this.groupTooltips[group.group_id];
        this.initPopper(image, tooltip);
      }
      this.initPopper(this.$refs.createButton, this.$refs.createTooltip);
    });
  },

  methods: {
    initPopper(button, tooltip) {
      button.onmouseenter = button.onfocus = () => {
        tooltip.setAttribute("data-show", "");
        this.poppers.push(createPopper(button, tooltip, {
          placement: "right",
          modifiers: [
            arrow, offset,
            {
              name: "offset",
              options: {
                offset: [0, 8],
              }
            }
          ]
        }))
      };
      button.onmouseleave = button.onblur = () => {
        tooltip.removeAttribute("data-show");
        this.poppers.shift().destroy();
      };
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

.group-tooltip {
  background-color: $gray-900;
  color: $gray-100;
  padding: 2px 4px;
  border-radius: 4px;
  z-index: 1000;
  display: none;
}

.group-tooltip[data-show] {
  display: block;
}

.tooltip-arrow, .tooltip-arrow::before {
  position: absolute;
  width: 8px;
  height: 8px;
  z-index: -1;
}

.tooltip-arrow::before {
  content: "";
  transform: rotate(45deg);
  background: $gray-900;
}

.group-tooltip[data-popper-placement^='top'] > .tooltip-arrow {
  bottom: -4px;
}

.group-tooltip[data-popper-placement^='bottom'] > .tooltip-arrow {
  top: -4px;
}

.group-tooltip[data-popper-placement^='left'] > .tooltip-arrow {
  right: -4px;
}

.group-tooltip[data-popper-placement^='right'] > .tooltip-arrow {
  left: -4px;
}
</style>
