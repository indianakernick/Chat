import { DELETED_USER_INFO } from "@/components/Message";

export default {
  cache: {
    0: DELETED_USER_INFO
  },

  getUserInfo(userId) {
    if (!this.cache.hasOwnProperty(userId)) {
      this.cache[userId] = {
        user_id: userId,
        name: ""
      };

      const req = new XMLHttpRequest();

      req.onload = () => {
        this.cache[userId].name = req.response.name;
      };

      req.responseType = "json";
      req.open("GET", `/api/user/${userId}`);
      req.send();
    }

    return this.cache[userId];
  },
};
