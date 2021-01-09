<template>
  <ModalDialog :shown="true" :backdrop="false" @submitForm="$emit('createGroup', true)">
    <template v-slot:header>
      No groups!
    </template>

    <template v-slot:body>
      You're not a member of any groups. Accept an invitation or create your own
      group and invite others.
    </template>

    <template v-slot:footer>
      <input type="submit" class="btn btn-primary" value="Create group"/>
    </template>
  </ModalDialog>
</template>

<script>
import ModalDialog from "./ModalDialog.vue";

export default {
  name: "NoGroups",

  components: {
    ModalDialog
  },

  emits: [
    "createGroup"
  ]
};
</script>

<style>

</style>
