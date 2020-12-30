<template>
  <div class="group-title">
    <div class="group-name">{{ currentGroupName }}</div>
    <div class="group-edit">&nbsp;=</div>
  </div>
</template>

<script>
export default {
  name: "GroupTitle",

  props: {
    currentGroupName: String
  }
};
</script>

<style>
.group-title {
  padding: 8px;
  background-color: gray;
  border-bottom: 1px solid black;
  color: white;
  font-weight: 500;
  font-size: 1.2rem;
  display: flex;
  flex-direction: row;
  justify-content: space-between;
}

/* identical to channel-name */
.group-name {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.group-edit {
  color: lightgray;
  cursor: pointer;
}

.group-edit:hover {
  color: white;
}
</style>
