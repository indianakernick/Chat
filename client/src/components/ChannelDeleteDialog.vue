<template>
  <ModalDialog :shown="shown" @submitForm="submitForm">
    <template v-slot:header>
      Delete #{{ channelName }}
    </template>

    <template v-slot:body>
      <span>
        Are you sure you want to delete #{{ channelName }}?
        Doing so will delete all messages within the channel.
        This operation cannot be undone.
      </span>
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
      channelName: 0,
      shown: false,
      waiting: false
    }
  },

  methods: {
    show(channelId, channelName) {
      this.channelId = channelId;
      this.channelName = channelName;
      this.waiting = false;
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

    channelError() {
      this.shown = false;
    }
  }
};
</script>

<style>

</style>
