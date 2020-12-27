<template>
  <ModalDialog :shown="shown" @submitForm="submitForm">
    <template v-slot:header>
      Create a new channel
    </template>

    <template v-slot:body>
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
          :readonly="waiting"
          required
          placeholder="my-new-channel"
        />
      </div>
      <small class="form-text text-muted">
        Must be 1-32 characters long, unique within the group, and cannot contain spaces, # or @
      </small>
    </template>

    <template v-slot:footer>
      <input type="button" class="btn btn-secondary" @click="hide" value="Cancel" :disabled="waiting"/>
      <input type="submit" class="btn btn-primary" value="Create" :disabled="waiting"/>
    </template>
  </ModalDialog>
</template>

<script>
import ModalDialog from "./ModalDialog.vue";

const REGEX_SPACE = /\p{White_Space}+/gu;
const REGEX_BAD_CHARS = /[#@]+/g;

export default {
  name: "ChannelCreateDialog",

  components: {
    ModalDialog
  },

  emits: [
    "createChannel"
  ],

  data() {
    return {
      name: "",
      shown: false,
      waiting: false,
      invalid: false
    }
  },

  methods: {
    show() {
      this.waiting = false;
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
      this.waiting = true;
      this.$emit("createChannel", this.name);
    },

    channelCreated(name) {
      if (this.waiting && name === this.name) {
        this.shown = false;
        return true;
      } else {
        return false;
      }
    },

    channelError() {
      if (this.waiting) {
        this.waiting = false;
        this.invalid = true;
      }
    }
  }
};
</script>

<style>

</style>
