import { DELETED_USER_INFO } from "@/components/Message.vue";
import { comp32, comp48 } from "./ImageCompositor.js";

export default {
  cache: {
    0: DELETED_USER_INFO
  },

  getUserInfo(userId) {
    if (!this.cache.hasOwnProperty(userId)) {
      this.cache[userId] = {
        name: "",
        picture: "",
        picture32: ""
      };

      const req = new XMLHttpRequest();

      req.onload = () => {
        this.cache[userId].name = req.response.name;
        comp48.composite(req.response.picture, url => {
          this.cache[userId].picture = url;
        });
        comp32.composite(req.response.picture, url => {
          this.cache[userId].picture32 = url;
        });
      };

      req.responseType = "json";
      req.open("GET", `/api/user/${userId}`);
      req.send();
    }

    return this.cache[userId];
  },
};
