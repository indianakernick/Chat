<template>
  <ModalDialog :shown="shown" @submitForm="submitForm">
    <template v-slot:header>
      <template v-if="rename">
        Rename <em>#{{ originalName }}</em>
      </template>
      <template v-else>
        Create a new channel
      </template>
    </template>

    <template v-slot:body>
      <label :for="id">Channel name</label>
      <div class="input-group">
        <div class="input-group-prepend">
          <div class="input-group-text">#</div>
        </div>
        <input
          :id="id"
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
      <input type="submit" class="btn btn-primary" :value="submitTitle" :disabled="waiting"/>
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

  props: {
    rename: Boolean
  },

  emits: [
    "createChannel",
    "renameChannel"
  ],

  data() {
    return {
      name: "",
      originalName: "",
      shown: false,
      waiting: false,
      invalid: false,
      channelId: 0
    }
  },

  computed: {
    id() {
      return this.rename ? "channel-rename-input" : "channel-create-input";
    },

    submitTitle() {
      return this.rename ? "Rename" : "Create";
    }
  },

  methods: {
    show(channelId, name) {
      if (this.rename) {
        this.channelId = channelId;
        this.name = this.originalName = name;
      }
      this.waiting = false;
      this.invalid = false;
      this.shown = true;
      this.$nextTick(() => {
        const input = document.getElementById(this.id);
        input.focus();
        if (this.rename) {
          input.value = name;
          input.select();
        }
      });
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
      if (this.rename) {
        this.$emit("renameChannel", this.channelId, this.name);
      } else {
        this.$emit("createChannel", this.name);
      }
    },

    channelCreated(name) {
      if (this.waiting && name === this.name) {
        this.shown = false;
        return true;
      } else {
        return false;
      }
    },

    channelRenamed(channelId) {
      if (this.waiting && channelId === this.channelId) {
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
