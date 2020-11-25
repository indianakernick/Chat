<template>
  <div class="message">
    <span class="message-creation-time">{{formattedCreationTime}}</span>
    &nbsp;-&nbsp;
    <span class="message-content">{{content}}</span>
  </div>
</template>

<script>
export default {
  name: "Message",

  props: {
    creation_time: Number,
    content: String
  },

  computed: {
    // This depends on the current time.
    // I could recompute every minute or half-minute.
    // Or maybe do a setTimeout to update at the right time.

    formattedCreationTime() {
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

      const creation = new Date(this.creation_time * 1000);

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

</style>
