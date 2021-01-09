<template>
  <ModalDialog :shown="shown" @submitForm="submitForm">
    <template v-slot:header>
      Delete account
    </template>

    <template v-slot:body>
      <span>
        Are you sure you want to delete your account?
        Doing so will anonymize all of your messages.
        Your name and picture will be removed but the message contents will remain.
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
  name: "UserDeleteDialog",

  components: {
    ModalDialog
  },

  data() {
    return {
      shown: false,
      waiting: false
    }
  },

  methods: {
    show() {
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

      req.open("DELETE", `/api/user`);
      req.send();
    }
  }
};
</script>

<style>

</style>
