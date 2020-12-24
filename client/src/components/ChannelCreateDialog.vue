<template>
  <!--
  This is a combination of the modal dialog examples from Vue and Bootstrap
  https://codepen.io/team/Vue/pen/mdPoyvv
  https://getbootstrap.com/docs/4.1/components/modal/
  -->

  <transition name="fade">
    <div v-if="shown" class="modal-mask">
      <div class="modal-wrapper">
        <div class="modal-dialog">
          <div class="modal-content">
            <form @submit.prevent="submitForm">
              <div class="modal-header">
                <h5 class="modal-title">Create a new channel</h5>
              </div>

              <div class="modal-body">
                <label for="channel-name-input">Channel name</label>
                <div class="input-group">
                  <div class="input-group-prepend">
                    <div class="input-group-text">#</div>
                  </div>
                  <input
                    id="channel-name-input"
                    class="form-control"
                    :class="invalid ? 'is-invalid' : ''"
                    type="text"
                    maxlength="32"
                    pattern="[^#@\p{White_Space}]+"
                    @input="validate"
                    :readonly="readonly"
                    required
                    placeholder="my-new-channel"
                  />
                </div>
                <small class="form-text text-muted">
                  Must be 1-32 characters long, unique within the group, and cannot contain spaces, # or @
                </small>
              </div>

              <div class="modal-footer">
                <input type="button" class="btn btn-secondary" @click="hide" value="Cancel"/>
                <input type="submit" class="btn btn-primary" value="Create"/>
              </div>
            </form>
          </div>
        </div>
      </div>
    </div>
  </transition>
</template>

<script>
const REGEX_SPACE = /\p{White_Space}+/gu;
const REGEX_BAD_CHARS = /[#@]+/g;

export default {
  name: "ChannelCreateDialog",

  emits: [
    "createChannel"
  ],

  data() {
    return {
      name: "",
      shown: false,
      readonly: false,
      invalid: false
    }
  },

  methods: {
    show() {
      this.readonly = false;
      this.invalid = false;
      this.shown = true;
      this.$nextTick(() => document.getElementById("channel-name-input").focus());
    },

    hide() {
      this.shown = false;
    },

    validate(e) {
      this.name = e.target.value = e.target.value
        .replace(REGEX_SPACE, "-")
        .replace(REGEX_BAD_CHARS, "");
      this.invalid = false;
    },

    submitForm() {
      this.readonly = true;
      this.$emit("createChannel", this.name);
    },

    channelCreated(name) {
      if (this.readonly && name === this.name) {
        this.shown = false;
      }
    },

    channelError() {
      if (this.readonly) {
        this.readonly = false;
        this.invalid = true;
      }
    }
  }
};
</script>

<style>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.modal-mask {
  position: fixed;
  z-index: 9998;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: table;
}

.modal-wrapper {
  display: table-cell;
  vertical-align: middle;
}
</style>
