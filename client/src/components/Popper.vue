<template>
  <div class="popper" ref="popperElement" tabindex="-1">
    <div class="popper-arrow" data-popper-arrow/>
    <slot/>
  </div>
</template>

<script>
import { createPopper } from "@popperjs/core/lib/popper-lite";
import arrow from "@popperjs/core/lib/modifiers/arrow";
import offset from "@popperjs/core/lib/modifiers/offset";
import { Placement } from "@popperjs/core/lib/enums";

export default {
  name: "Popper",

  props: {
    placement: Placement,
    // HTML attribute values are strings
    distance: String,
    skid: {
      type: String,
      default: "0"
    },
    arrowPadding: {
      type: String,
      default: "0"
    }
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
              offset: [parseInt(this.skid), parseInt(this.distance)]
            }
          },
          {
            name: "arrow",
            options: {
              padding: parseInt(this.arrowPadding)
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
    },

    initDropdownButton(button) {
      button.onmousedown = e => {
        if (button.hasAttribute("data-active")) {
          e.preventDefault();
        }
      };
      button.onclick = e => {
        e.stopPropagation();
        if (this.toggle(button)) {
          this.$el.focus();
          button.setAttribute("data-active", "");
        } else {
          this.hide();
          button.removeAttribute("data-active");
        }
      };
      this.$el.onblur = () => {
        this.hide();
        button.removeAttribute("data-active");
      };
    },

    initTooltipButton(button) {
      button.onmouseenter = () => {
        this.show(button);
      };
      button.onmouseleave = () => {
        this.hide();
      };
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

.popper:focus {
  outline: none;
}

.popper.tooltip {
  background-color: $tooltip-back;
  color: $tooltip-text;
  z-index: 200;
  padding: 4px 8px;
  font-weight: 500;
}

.popper.dropdown {
  background-color: $dropdown-back;
  color: $dropdown-text;
  z-index: 100;
  width: calc(100% - 16px)
}

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
  background-color: $dropdown-item-hover-back;
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
  background-color: $tooltip-back;
}

.popper.dropdown .popper-arrow::before {
  background-color: $dropdown-back;
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
