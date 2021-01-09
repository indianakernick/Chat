<template>
  <div class="message" :class="{'sending': sending}">
    <img
      class="user-picture"
      :src="userInfo.picture48"
      alt="User picture"
      width="48"
      height="48"
    />
    <div class="message-right">
      <div>
        <span class="user-name" :class="{'deleted': deleted}">{{ userInfo.name }}</span>
        &nbsp;<span class="message-time">{{ formattedTime }}</span>
      </div>
      <span class="message-content">{{ content }}</span>
    </div>
  </div>
</template>

<script>
import Anonymous from "@/assets/anonymous.png";

export const DELETED_USER_INFO = {
  name: "<anonymous>",
  picture: "",
  picture32: "",
  picture48: Anonymous,
  deleted: true
};

const timeFormatManager = {
  timeFormatter: new Intl.DateTimeFormat([], {
    hour: "2-digit",
    minute: "2-digit"
  }),

  dateTimeFormatter: new Intl.DateTimeFormat([], {
    day: "2-digit",
    month: "short",
    hour: "2-digit",
    minute: "2-digit"
  }),

  yearDateTimeFormatter: new Intl.DateTimeFormat([], {
    year: "numeric",
    month: "short",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit"
  }),

  initialized: false,
  todayMessages: { date: null, messages: new Set() },
  thisYearMessages: { date: null, messages: new Set() },
  today: null,
  thisYear: null,

  initialize() {
    if (!this.initialized) {
      this.initialized = true;
      this.update();
      window.timeFormatManager = this;
    }
  },

  update() {
    const now = Date.now();

    this.today = new Date(now);
    const today = this.today.setHours(0, 0, 0, 0);
    const tomorrow = this.today.setDate(this.today.getDate() + 1);
    this.today = today;

    this.thisYear = new Date(this.today);
    const thisYear = this.thisYear.setMonth(0, 1);
    const nextYear = this.thisYear.setFullYear(this.thisYear.getFullYear() + 1);
    this.thisYear = thisYear;

    if (this.todayMessages.date <= now) {
      this.updateMessages(this.todayMessages, tomorrow);
    }

    if (this.thisYearMessages.date <= now) {
      this.updateMessages(this.thisYearMessages, nextYear);
    }

    let nextMinute = new Date(now);
    nextMinute = nextMinute.setMinutes(nextMinute.getMinutes() + 1, 0, 0);
    setTimeout(() => this.update(), nextMinute - now);
  },

  updateMessages(messages, newDate) {
    messages.date = newDate;
    for (const message of messages.messages) {
      message.formattedTime = this.formatTime(message);
    }
  },

  removeFromList(message) {
    this.todayMessages.messages.delete(message);
    this.thisYearMessages.messages.delete(message);
  },

  formatTime(message) {
    this.removeFromList(message);
    const time = new Date(message.timestamp * 1000);

    if (time >= this.today) {
      this.todayMessages.messages.add(message);
      return this.timeFormatter.format(time);
    }

    if (time >= this.thisYear) {
      this.thisYearMessages.messages.add(message);
      return this.dateTimeFormatter.format(time);
    }

    return this.yearDateTimeFormatter.format(time);
  },
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
    timeFormatManager.initialize();
    return {
      formattedTime: timeFormatManager.formatTime(this)
    }
  },

  beforeUnmount() {
    timeFormatManager.removeFromList(this);
  },

  watch: {
    timestamp() {
      this.formattedTime = timeFormatManager.formatTime(this);
    }
  },

  computed: {
    deleted() {
      return this.userInfo.hasOwnProperty("deleted");
    }
  }
};
</script>

<style lang="scss">
@import "../scss/colors";

$padding: 8px;

.message {
  display: flex;
  padding: $padding $padding 0 $padding;
}

.message:last-child {
  padding-bottom: $padding;
}

.message .user-picture {
  flex: 0 0 48px;
}

.message-time {
  color: $message-time-text;
  font-size: 0.8rem;
}

.message-content {
  color: $message-content-text;
}

.sending span {
  color: $message-sending-text;
}

.message-right {
  display: flex;
  flex-direction: column;
}
</style>
