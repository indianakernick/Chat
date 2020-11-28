<template>
  <div class="message" :class="{'sending': sending}">
    <span class="message-time">{{formattedTime}}</span>
    -
    <span class="message-author">{{author}}</span>
    -
    <span class="message-content">{{content}}</span>
  </div>
</template>

<script>
export default {
  name: "Message",

  props: {
    timestamp: Number,
    author: Number,
    content: String,
    sending: Boolean
  },

  computed: {
    // TODO: Use setTimeout to update the representation
    formattedTime() {
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

      const time = new Date(this.timestamp * 1000);

      const dayStart = new Date();
      dayStart.setHours(0, 0, 0, 0);
      if (time >= dayStart) {
        return timeFormatter.format(time);
      }

      const yearStart = new Date(dayStart.getTime());
      yearStart.setMonth(0, 1);
      if (time >= yearStart) {
        return dateTimeFormatter.format(time);
      }

      return yearDateTimeFormatter.format(time);
    }
  }
};
</script>

<style scoped>
  .sending {
    color: #555;
  }
</style>
