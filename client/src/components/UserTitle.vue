<template>
  <div class="user-title">
    <!-- https://stackoverflow.com/a/61042200/4093378 -->
    <!--<img :src="userInfo.picture" alt="" width="32" height="32" referrerpolicy="no-referrer"/>-->
    <div class="user-name">{{ userInfo.name }}</div>
    <div class="user-edit">&nbsp;=</div>
  </div>
</template>

<script>
export default {
  name: "UserTitle",

  props: {
    userInfo: Object
  }
};
</script>

<style>
/* This is pretty much ripped straight from group title */

.user-title {
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

/* This has been copied and pasted in lots of places */
.user-name {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.user-edit {
  color: lightgray;
  cursor: pointer;
}

.user-edit:hover {
  color: white;
}
</style>
