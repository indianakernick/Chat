<template>
  <ModalDialog :shown="shown" @submitForm="submitForm">
    <template v-slot:header>
      Invite people to <em>{{ groupName }}</em>
    </template>

    <template v-slot:body>
      <label for="link-input">Invitation link</label>
      <div class="input-group">
        <input
          id="link-input"
          class="form-control"
          type="text"
          readonly
          placeholder="Loading..."
          :value="link"
          @focus="selectAll"
          @mouseup.prevent
        />
        <div class="input-group-append">
          <input type="button" class="btn btn-primary" @click="copy" value="Copy" :disabled="waiting"/>
        </div>
      </div>
      <small class="form-text text-muted">
        Anyone with this link will be able to join this group within the next 24 hours.
      </small>
    </template>

    <template v-slot:footer>
      <input type="button" class="btn btn-secondary" @click="hide" value="Dismiss" :disabled="waiting"/>
    </template>
  </ModalDialog>
</template>

<script>
import ModalDialog from "./ModalDialog.vue";

export default {
  name: "InviteDialog",

  components: {
    ModalDialog
  },

  props: {
    groupId: Number,
    groupName: String
  },

  data() {
    return {
      shown: false,
      waiting: true,
      link: ""
    }
  },

  methods: {
    show() {
      this.waiting = true;
      this.link = "";
      this.shown = true;

      const req = new XMLHttpRequest();

      req.onload = () => {
        if (this.waiting) {
          console.log(req.response);
          this.link = window.location.origin + "/invite/" + req.response.invite_id;
          this.waiting = false;
        }
      };

      req.responseType = "json";
      req.open("POST", "/api/invite");
      req.setRequestHeader("Content-Type", "application/json;charset=UTF-8");
      req.send(`{"group_id":${this.groupId}}`);
    },

    hide() {
      this.shown = false;
    },

    copy() {
      if (this.link.length > 0) {
        navigator.clipboard.writeText(this.link);
      }
    },

    selectAll(e) {
      e.target.select();
    },

    submitForm() {}
  }
};
</script>

<style>

</style>
