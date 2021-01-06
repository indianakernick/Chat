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
        Must be 1-32 characters long, and unique
      </small>

      <label for="group-picture-input">Group image</label>
      <input
        id="group-picture-input"
        class="form-control"
        :class="invalidPicture ? 'is-invalid' : ''"
        type="file"
        accept="image/*"
        required
        :readonly="waiting"
        @change="changePicture"
      />

      <img class="group-list-item" width="64" height="64" :src="pictureSrc"/>

    </template>

    <template v-slot:footer>
      <input type="button" class="btn btn-secondary" @click="hide" value="Cancel" :disabled="waiting"/>
      <input type="submit" class="btn btn-primary" :value="submitTitle" :disabled="waiting"/>
    </template>
  </ModalDialog>
</template>

<script>
import ModalDialog from "./ModalDialog.vue";
import ImageCompositor from "@/assets/js/ImageCompositor.js";

const comp64 = new ImageCompositor(64, "#e9ecef"); // $group-item-back

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
      rename: false,
      pictureSrc: "",
      pictureBlob: null
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

    // TODO: Do we really need to pass in the name?
    // Why not use a prop?
    showRename(name) {
      this.show(true);
      this.name = this.originalName = name;
      this.$nextTick(() => {
        const input = document.getElementById("group-name-input");
        input.focus();
        input.value = name;
        input.select();
      });
    },

    hide() {
      this.shown = false;
    },

    submitForm() {
      this.waiting = true;
      if (this.rename) {
        this.$emit("renameGroup", this.name);
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
            this.handleError(req.response.message);
          } else if (req.response.type === "success") {
            this.handleSuccess(req.response.group_id);
          }
        }
      };

      req.responseType = "json";
      req.open("POST", "/api/group/create");
      const form = new FormData();
      form.append("name", this.name);
      form.append("picture", this.pictureBlob);
      req.send(form);
    },

    handleError(message) {
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

        case "request_invalid":
        case "fs":
          // Not really sure what to do in this situation
          this.invalidName = true;
          this.invalidPicture = true;
          break;
      }
    },

    handleSuccess(groupId) {
      this.shown = false;
      this.$emit("createGroup", {
        group_id: groupId, name: this.name
      });
    },

    groupRenamed(name) {
      if (this.waiting && this.name === name) {
        this.shown = false;
      } else {
        this.originalName = name;
      }
    },

    error(code) {
      if (this.waiting) {
        this.waiting = false;
        switch (code) {
          case "name_invalid":
          case "name_exists":
            this.invalidName = true;
            this.invalidPicture = false;
            break;
        }
      }
    },

    changePicture(e) {
      if (e.target.files.length === 0) {
        URL.revokeObjectURL(this.pictureSrc);
        this.pictureSrc = "";
        return;
      }
      const originalURL = URL.createObjectURL(e.target.files[0]);
      comp64.composite(originalURL, blob => {
        URL.revokeObjectURL(originalURL);
        URL.revokeObjectURL(this.pictureSrc);
        this.pictureSrc = URL.createObjectURL(blob);
        this.pictureBlob = blob;
      });
    }
  }
};
</script>

<style>

</style>
