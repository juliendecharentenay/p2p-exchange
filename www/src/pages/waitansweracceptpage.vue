<template>
  <div class="absolute inset-0 flex flex-col justify-center content-center gap-y-2">
    <p class="mt-6 text-lg leading-8 text-gray-600 text-center">
    Waiting for connection
    </p>
    <div class="mt-4 flex flex-row gap-x-2 justify-center">
      <PhoneArrowDownLeftIcon class="w-20 h-20 animate-pulse text-indigo-600" />
    </div>
  </div>
</template>
<script>
import { PhoneArrowDownLeftIcon } from "@heroicons/vue/24/outline";
export default {
  name: "OfferPage",
  emits: [ 'processing', 'error', 'to' ],
  components: {
    PhoneArrowDownLeftIcon,
  },
  methods: {
    on_error: function(message, error) {
      this.$emit('error', {error, message});
    }
  }
}
</script>
