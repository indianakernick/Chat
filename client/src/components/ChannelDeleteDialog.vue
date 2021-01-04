<template>
  <ModalDialog :shown="shown" @submitForm="submitForm">
    <template v-slot:header>
      Delete <em>#{{ name }}</em>
    </template>

    <template v-slot:body>
      <span :class="errorMessage.length > 0 ? 'is-invalid' : ''">
        Are you sure you want to delete <em>#{{ name }}</em>?
        Doing so will delete all messages within the channel.
        This operation cannot be undone.
      </span>
      <div class="invalid-feedback">
        {{ errorMessage }}
      </div>
    </template>

    <template v-slot:footer>
      <input type="button" class="btn btn-secondary" @click="hide" value="Cancel" :disabled="waiting"/>
      <input type="submit" class="btn btn-primary" value="Delete" :disabled="waiting"/>
    </template>
  </ModalDialog>
</template>

<script>
import ModalDialog from "./ModalDialog.vue";

export default {
  name: "ChannelDeleteDialog",

  components: {
    ModalDialog
  },

  emits: [
    "deleteChannel"
  ],

  data() {
    return {
      channelId: -1,
      name: "",
      shown: false,
      waiting: false,
      errorMessage: ""
    }
  },

  methods: {
    show(channelId, name) {
      this.channelId = channelId;
      this.name = name;
      this.waiting = false;
      this.errorMessage = "";
      this.shown = true;
    },

    hide() {
      this.shown = false;
    },

    submitForm() {
      this.waiting = true;
      this.$emit("deleteChannel", this.channelId);
    },

    channelDeleted(channelId) {
      if (this.waiting && channelId === this.channelId) {
        this.shown = false;
      }
    },

    channelRenamed(channelId, name) {
      if (channelId === this.channelId) {
        this.name = name;
      }
    },

    channelError(error) {
      this.waiting = false;
      if (error === "Cannot delete lone channel") {
        this.errorMessage = "You cannot delete a channel if it is the only channel in a group.";
      } else {
        this.errorMessage = "Error occurred while trying to delete channel.";
      }
    }
  }
};
</script>

<style>

</style>
