<template>
  <div class="message" :class="{'sending': sending}">
    <span class="message-time">{{formattedTime}}</span>
    &nbsp;-&nbsp;
    <span class="message-content">{{content}}</span>
  </div>
</template>

<script>
export default {
  name: "Message",

  props: {
    timestamp: Number,
    content: String,
    sending: Boolean
  },

  computed: {
    // This depends on the current time.
    // I could recompute every minute or half-minute.
    // Or maybe do a setTimeout to update at the right time.

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

      const creation = new Date(this.timestamp * 1000);

      const dayStart = new Date();
      dayStart.setHours(0, 0, 0, 0);
      if (creation >= dayStart) {
        return timeFormatter.format(creation);
      }

      const yearStart = new Date(dayStart.getTime());
      yearStart.setMonth(0, 1);
      if (creation >= yearStart) {
        return dateTimeFormatter.format(creation);
      }

      return yearDateTimeFormatter.format(creation);
    }
  }
};
</script>

<style scoped>
  .sending {
    color: #555;
  }
</style>
