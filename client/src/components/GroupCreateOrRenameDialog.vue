<template>
  <ModalDialog :shown="shown" @submitForm="submitForm">
    <template v-slot:header>
      <template v-if="rename">
        Rename <em>{{ originalName }}</em>
      </template>
      <template v-else>
        Create a new group
      </template>
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
        Must be 1-32 characters, and unique
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
        Must be 1-2048 characters
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

export default {
  name: "GroupCreateDialog",

  components: {
    ModalDialog
  },

  data() {
    return {
      name: "",
      originalName: "",
      picture: "",
      shown: false,
      waiting: false,
      invalidName: false,
      invalidPicture: false,
      rename: false
    }
  },

  emits: [
    "createGroup",
    "renameGroup"
  ],

  computed: {
    submitTitle() {
      return this.rename ? "Rename" : "Create";
    }
  },

  methods: {
    show(rename) {
      this.rename = rename;
      this.waiting = false;
      this.invalid = false;
      this.shown = true;
    },

    showCreate() {
      this.show(false);
      this.name = "";
      this.$nextTick(() => {
        const input = document.getElementById("group-name-input");
        input.focus();
        input.value = "";
        document.getElementById("group-picture-input").value = "";
      });
    },

    showRename(name, picture) {
      this.show(true);
      this.name = this.originalName = name;
      this.picture = picture;
      this.$nextTick(() => {
        const input = document.getElementById("group-name-input");
        input.focus();
        input.value = name;
        input.select();
        document.getElementById("group-picture-input").value = picture;
      });
    },

    hide() {
      this.shown = false;
    },

    submitForm() {
      this.waiting = true;
      if (this.rename) {
        this.$emit("renameGroup", this.name, this.picture);
      } else {
        this.createGroup();
      }
    },

    createGroup() {
      const req = new XMLHttpRequest();

      req.onload = () => {
        if (this.waiting) {
          console.log(req.response);
          if (req.response.type === "error") {
            this.error(req.response.message);
          } else if (req.response.type === "success") {
            this.success(req.response.group_id);
          }
        }
      };

      req.responseType = "json";
      req.open("POST", "/api/group");
      req.setRequestHeader("Content-Type", "application/json;charset=UTF-8");
      req.send(JSON.stringify({
        name: this.name,
        picture: this.picture
      }));
    },

    error(message) {
      if (this.waiting) {
        this.waiting = false;
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
    },

    success(groupId) {
      this.shown = false;
      this.$emit("createGroup", {
        group_id: groupId, name: this.name, picture: this.picture
      });
    },

    groupRenamed(name, picture) {
      if (this.waiting && this.name === name && this.picture === picture) {
        this.shown = false;
      } else {
        this.originalName = name;
      }
    },

    groupDeleted() {
      this.shown = false;
    }
  }
};
</script>

<style>

</style>
