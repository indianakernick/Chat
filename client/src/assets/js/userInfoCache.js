import { DELETED_USER_INFO } from "@/components/Message.vue";
import { comp32, comp48 } from "./ImageCompositor.js";
import { reactive, watchEffect } from "vue";

export default {
  cache: {
    0: DELETED_USER_INFO
  },

  createReactiveUser(name, picture) {
    const reactiveUser = reactive({
      name: name,
      picture: picture,
      picture32: "",
      picture48: ""
    });
    watchEffect(() => {
      comp32.composite(reactiveUser.picture, url => {
        reactiveUser.picture32 = url;
      });
      comp48.composite(reactiveUser.picture, url => {
        reactiveUser.picture48 = url;
      });
    });
    return reactiveUser;
  },

  getUserInfo(userId) {
    if (!this.cache.hasOwnProperty(userId)) {
      this.cache[userId] = this.createReactiveUser("", "");

      const req = new XMLHttpRequest();

      req.onload = () => {
        this.cache[userId].name = req.response.name;
        this.cache[userId].picture = req.response.picture;
      };

      req.responseType = "json";
      req.open("GET", `/api/user/${userId}`);
      req.send();
    }

    return this.cache[userId];
  },

  setUserInfo(userId, name, picture) {
    if (!this.cache.hasOwnProperty(userId)) {
      this.cache[userId] = this.createReactiveUser(name, picture);
    } else {
      this.cache[userId].name = name;
      this.cache[userId].picture = picture;
    }
  },

  removeUserInfo(userId) {
    delete this.cache[userId];
  }
};
