<template>
  <!--
  TODO: Put the modal dialog code into one component and use slots to populate
  it. This might actually be quite difficult because the modal dialog and the
  content it contains are intertwined.
  -->

  <transition name="fade">
    <div v-if="shown" class="modal-mask">
      <div class="modal-wrapper">
        <div class="modal-dialog">
          <div class="modal-content">
            <form @submit.prevent="submitForm">
              <div class="modal-header">
                <h5 class="modal-title">Delete #{{ channelName }}</h5>
              </div>

              <div class="modal-body">
                <span>
                  Are you sure you want to delete #{{ channelName }}?
                  Doing so will delete all messages within the channel.
                  This operation cannot be undone.
                </span>
              </div>

              <div class="modal-footer">
                <input type="button" class="btn btn-secondary" @click="hide" value="Cancel"/>
                <input type="submit" class="btn btn-primary" value="Delete" :disabled="waiting"/>
              </div>
            </form>
          </div>
        </div>
      </div>
    </div>
  </transition>
</template>

<script>
export default {
  name: "ChannelDeleteDialog",

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
