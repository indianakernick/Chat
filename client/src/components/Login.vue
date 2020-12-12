<template>
  You must login before you can use this app.
  <a :href="url">Login with Google</a>
</template>

<script>
// TODO: Should the login page use Vue at all?
// It probably doesn't need to. Askama alone would suffice.

import ClientID from "./../../../api/client_id.txt"

export default {
  name: "Login",

  data() {
    return {
      url: "https://accounts.google.com/o/oauth2/v2/auth?"
        + "client_id=" + ClientID + "&"
        + "redirect_uri=https://localhost/api/auth&"
        + "response_type=code&"
        + "scope=profile&"
        + "state=" + encodeURIComponent(REDIRECT_URL)
    }
  }
};
</script>

<style scoped>

</style>
