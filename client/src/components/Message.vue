<template>
  <div class="message" :class="{'sending': sending}">
    <img class="message-picture" alt="" width="32" height="32" :src="userInfo.picture"/>
    <div class="message-right">
      <div>
        <span class="message-name" :class="{'deleted': deleted}">{{userInfo.name}}</span>
        -
        <span class="message-time">{{formattedTime}}</span>
      </div>
      <span class="message-content">{{content}}</span>
    </div>
  </div>
</template>

<script>
export const DELETED_USER_INFO = {
  name: "<deleted user>",
  picture: "https://www.gravatar.com/avatar/00000000000000000000000000000000?d=mp&f=y&s=32",
  deleted: true
};

export default {
  name: "Message",

  props: {
    timestamp: Number,
    content: String,
    sending: Boolean,
    userInfo: Object
  },

  data() {
    return {
      timeoutId: -1,
      formattedTime: this.formatTime(),
      updateTime: true,
      deleted: this.userInfo.hasOwnProperty("deleted")
    }
  },

  created() {
    this.$watch(
        () => [this.timestamp, this.updateTime],
        () => this.formattedTime = this.formatTime()
    );
  },

  beforeUnmount() {
    clearTimeout(this.timeoutId);
  },

  methods: {
    formatTime() {
      clearTimeout(this.timeoutId);

      const timeFormatter = new Intl.DateTimeFormat([], {
        hour: "2-digit",
        minute: "2-digit"
      });
      const dateTimeFormatter = new Intl.DateTimeFormat([], {
        day: "2-digit",
        month: "short",
        hour: "2-digit",
        minute: "2-digit"
      });
      const yearDateTimeFormatter = new Intl.DateTimeFormat([], {
        year: "numeric",
        month: "short",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit"
      });

      const now = new Date();
      const time = new Date(this.timestamp * 1000);

      const dayStart = new Date(now.getTime());
      dayStart.setHours(0, 0, 0, 0);
      if (time >= dayStart) {
        dayStart.setDate(dayStart.getDate() + 1);
        const delay = dayStart.getTime() - now.getTime();
        this.timeoutId = setTimeout(() => {
          this.updateTime = !this.updateTime;
        }, delay);
        return timeFormatter.format(time);
      }

      const yearStart = dayStart;
      yearStart.setMonth(0, 1);
      if (time >= yearStart) {
        yearStart.setFullYear(yearStart.getFullYear() + 1);
        const delay = Math.min(2 ** 31 - 1, yearStart.getTime() - now.getTime());
        this.timeoutId = setTimeout(() => {
          this.updateTime = !this.updateTime;
        }, delay);
        return dateTimeFormatter.format(time);
      }

      this.timeoutId = -1;
      return yearDateTimeFormatter.format(time);
    }
  }
};
</script>

<style scoped>
.message {
  display: flex;
  flex-direction: row;
}

.message-right {
  display: flex;
  flex-direction: column;
}

.sending {
  color: #555;
}

.deleted {
  color: #F00;
}
</style>
