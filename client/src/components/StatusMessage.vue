<template>
  <div>
    <span>{{status}}</span>
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

<style scoped>
div {
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
}

div > span {
  text-align: center;
}
</style>