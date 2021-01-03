<template>
  <div class="popper" ref="popperElement">
    <div class="popper-arrow" data-popper-arrow/>
    <slot/>
  </div>
</template>

<script>
import {createPopper} from "@popperjs/core/lib/popper-lite";
import arrow from "@popperjs/core/lib/modifiers/arrow";
import offset from "@popperjs/core/lib/modifiers/offset";
import {Placement} from "@popperjs/core/lib/enums";

export default {
  name: "Popper",

  props: {
    placement: Placement,
    // HTML attribute values are strings
    offset: {}
  },

  data() {
    return {
      popper: null,
      config: {
        placement: this.placement,
        modifiers: [
          arrow, offset,
          {
            name: "offset",
            options: {
              offset: [0, this.offset],
            }
          }
        ]
      }
    }
  },

  methods: {
    show(reference) {
      if (!this.popper) {
        this.$refs.popperElement.setAttribute("data-show", "");
        this.popper = createPopper(reference, this.$refs.popperElement, this.config);
      }
    },

    hide() {
      if (this.popper) {
        this.$refs.popperElement.removeAttribute("data-show");
        this.popper.destroy();
        this.popper = null;
      }
    },

    toggle(reference) {
      if (this.popper) {
        this.hide();
        return false;
      } else {
        this.show(reference);
        return true;
      }
    }
  }
};
</script>

<style lang="scss">
@import "../scss/colors";

.popper {
  border-radius: 4px;
  display: none;
}

.popper.tooltip {
  background-color: black;
  color: white;
  z-index: 200;
  padding: 4px 8px;
  font-weight: 500;
}

.popper.dropdown {
  background-color: $gray-900;
  color: $gray-300;
  z-index: 100;
}

.popper[data-show] {
  display: block;
}

// This is mostly based on the tutorial example
// https://popper.js.org/docs/v2/tutorial/

.popper-arrow, .popper-arrow::before {
  position: absolute;
  width: 8px;
  height: 8px;
  z-index: -1;
}

.popper-arrow::before {
  content: "";
  transform: rotate(45deg);
}

.popper.tooltip .popper-arrow::before {
  background: black;
}

.popper.dropdown .popper-arrow::before {
  background: $gray-900;
}

.popper[data-popper-placement^="top"] > .popper-arrow {
  bottom: -4px;
}

.popper[data-popper-placement^="bottom"] > .popper-arrow {
  top: -4px;
}

.popper[data-popper-placement^="left"] > .popper-arrow {
  right: -4px;
}

.popper[data-popper-placement^="right"] > .popper-arrow {
  left: -4px;
}
</style>
