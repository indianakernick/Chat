<template>
  <div class="status">
    <span>{{ status }}</span>
  </div>
</template>

<script>
export default {
  name: "StatusMessage",

  props: {
    status: String
  }
};
</script>

<style>
.status {
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
}

.status > span {
  color: white;
  font-size: 2rem;
}
</style>
