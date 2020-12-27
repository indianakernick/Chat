<template>
  <ModalDialog :shown="shown" @submitForm="submitForm">
    <template v-slot:header>
      Create a new group
    </template>

    <template v-slot:body>
      <label for="group-name-input">Group name</label>
      <input
        id="group-name-input"
        class="form-control"
        :class="invalidName ? 'is-invalid' : ''"
        type="text"
        maxlength="32"
        :readonly="waiting"
        required
        placeholder="My New Group"
        v-model="name"
      />
      <small class="form-text text-muted">
        Must be 1-32 characters long, and unique
      </small>

      <label for="group-picture-input">Group image URL</label>
      <input
        id="group-picture-input"
        class="form-control"
        :class="invalidPicture ? 'is-invalid' : ''"
        type="url"
        maxlength="2048"
        :readonly="waiting"
        placeholder="http://somesite/someimage.png"
        v-model="picture"
      />
      <small class="form-text text-muted">
        Must be 1-2048 characters, or empty
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

export default {
  name: "GroupCreateDialog",

  components: {
    ModalDialog
  },

  data() {
    return {
      name: "",
      picture: "",
      shown: false,
      waiting: false,
      invalidName: false,
      invalidPicture: false
    }
  },

  emits: [
    "createGroup"
  ],

  methods: {
    show() {
      this.waiting = false;
      this.invalidName = false;
      this.invalidPicture = false;
      this.shown = true;
      this.$nextTick(() => document.getElementById("group-name-input").focus());
    },

    hide() {
      this.shown = false;
    },

    submitForm() {
      this.waiting = true;

      const req = new XMLHttpRequest();

      req.onload = () => {
        if (this.waiting) {
          console.log(req.response);
          if (req.response.type === "error") {
            this.handleError(req.response.message);
          } else if (req.response.type === "success") {
            this.handleSuccess(req.response.group_id);
          }
        }
      };

      req.responseType = "json";
      req.open("POST", "/api/group/create");
      req.setRequestHeader("Content-Type", "application/json;charset=UTF-8");
      req.send(JSON.stringify({
        name: this.name,
        picture: this.picture
      }));
    },

    handleError(message) {
      this.waiting = false;
      switch (message) {
        case "Invalid group name":
        case "Duplicate group name":
          this.invalidName = true;
          this.invalidPicture = false;
          break;

        case "Invalid url":
          this.invalidName = false;
          this.invalidPicture = true;
          break;
      }
    },

    handleSuccess(groupId) {
      this.shown = false;
      this.$emit("createGroup", {
        group_id: groupId, name: this.name, picture: this.picture
      });
    }
  }
};
</script>

<style>

</style>
