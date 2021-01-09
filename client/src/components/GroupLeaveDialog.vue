<template>
  <ModalDialog :shown="shown" @submitForm="submitForm">
    <template v-slot:header>
      Leave <em>{{ name }}</em>
    </template>

    <template v-slot:body>
      <span>
        Are you sure you want to leave <em>{{ name }}</em>?
        Doing so will anonymize all of your messages.
        Your name and picture will be removed but the message contents will remain.
        If you choose the join the group again your messages will still be anonymous.
      </span>
    </template>

    <template v-slot:footer>
      <input type="button" class="btn btn-secondary" @click="hide" value="Cancel" :disabled="waiting"/>
      <input type="submit" class="btn btn-primary" value="Leave" :disabled="waiting"/>
    </template>
  </ModalDialog>
</template>

<script>
import ModalDialog from "./ModalDialog.vue";

export default {
  name: "GroupLeaveDialog",

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

      req.open("POST", `/api/leave/${this.groupId}`);
      req.send();
    }
  }
};
</script>

<style>

</style>
