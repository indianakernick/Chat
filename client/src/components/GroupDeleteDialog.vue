<template>
  <ModalDialog :shown="shown" @submitForm="submitForm">
    <template v-slot:header>
      Delete <em>{{ name }}</em>
    </template>

    <template v-slot:body>
      <span>
        Are you sure you want to delete <em>{{ name }}</em>?
        Doing so will delete all channels and all messages within the group.
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
  name: "GroupDeleteDialog",

  components: {
    ModalDialog
  },

  data() {
    return {
      groupId: 0,
      name: "",
      shown: false,
      waiting: false
    }
  },

  methods: {
    show(groupId, name) {
      this.groupId = groupId;
      this.name = name;
      this.waiting = false;
      this.shown = true;
    },

    hide() {
      this.shown = false;
    },

    submitForm() {
      this.waiting = true;

      const req = new XMLHttpRequest();

      req.onload = () => {
        this.shown = false;
      };

      req.open("DELETE", `/api/group/${this.groupId}`);
      req.send();
    },

    groupRenamed(name) {
      this.name = name;
    },

    groupDeleted() {
      this.shown = false;
    }
  }
};
</script>

<style>

</style>
