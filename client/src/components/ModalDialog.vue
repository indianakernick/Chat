<template>
  <!--
  This is a combination of the modal dialog examples from Vue and Bootstrap
  https://codepen.io/team/Vue/pen/mdPoyvv
  https://getbootstrap.com/docs/4.1/components/modal/
  -->

  <transition name="fade">
    <div v-if="shown" class="modal-mask">
      <div class="modal-wrapper">
        <div class="modal-dialog">
          <div class="modal-content">
            <form @submit.prevent="submitForm">
              <div class="modal-header">
                <h5 class="modal-title">
                  <slot name="header"></slot>
                </h5>
              </div>
              <div class="modal-body">
                <slot name="body"></slot>
              </div>
              <div class="modal-footer">
                <slot name="footer"></slot>
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
  name: "ModalDialog",

  emits: [
    "submitForm"
  ],

  props: {
    shown: Boolean
  },

  methods: {
    submitForm() {
      this.$emit("submitForm");
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
