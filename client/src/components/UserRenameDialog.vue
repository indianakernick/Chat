<template>
  <ModalDialog :shown="shown" @submitForm="submitForm">
    <template v-slot:header>
      Change name
    </template>

    <template v-slot:body>
      <label for="user-name-input">User name</label>
      <input
        id="user-name-input"
        class="form-control"
        :class="invalidName ? 'is-invalid' : ''"
        type="text"
        maxlength="64"
        :readonly="waiting"
        required
        placeholder="John Appleseed"
        v-model="name"
      />
      <small class="form-text text-muted">
        Must be 1-64 characters, and unique
      </small>

      <label for="user-picture-input">User picture URL</label>
      <input
        id="user-picture-input"
        class="form-control"
        :class="invalidPicture ? 'is-invalid' : ''"
        type="url"
        maxlength="2048"
        :readonly="waiting"
        placeholder="http://somesite/someimage.png"
        v-model="picture"
      />
      <small class="form-text text-muted">
        Must be 1-2048 characters
      </small>
    </template>

    <template v-slot:footer>
      <input type="button" class="btn btn-secondary" @click="hide" value="Cancel" :disabled="waiting"/>
      <input type="submit" class="btn btn-primary" value="Rename" :disabled="waiting"/>
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

  methods: {
    show(name, picture) {
      this.waiting = false;
      this.invalid = false;
      this.name = name;
      this.picture = picture;
      this.shown = true;
      this.$nextTick(() => {
        const input = document.getElementById("user-name-input");
        input.focus();
        input.value = name;
        input.select();
        document.getElementById("user-picture-input").value = picture;
      });
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
          if (req.response.length) {
            this.waiting = false;
            this.error(req.response);
          } else {
            this.shown = false;
          }
        }
      };

      req.responseType = "text";
      req.open("PUT", "/api/user");
      req.setRequestHeader("Content-Type", "application/json;charset=UTF-8");
      req.send(JSON.stringify({
        name: this.name,
        picture: this.picture
      }));
    },

    error(message) {
      switch (message) {
        case "name_invalid":
        case "name_exists":
          this.invalidName = true;
          this.invalidPicture = false;
          break;
        case "picture_invalid":
          this.invalidName = false;
          this.invalidPicture = true;
          break;
      }
    }
  }
};
</script>

<style>

</style>
