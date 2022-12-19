<template>
  <div class="absolute inset-0 flex flex-col justify-center content-center gap-y-2">
    <p class="mt-6 text-lg leading-8 text-gray-600 sm:text-center">
    The offer code is:
    </p>
    <div class="mt-4 flex flex-row justify-center">
      {{ offer_id }}
    </div>
    <div class="mt-8 flex flex-row justify-center">
      <PhoneArrowUpRightIcon class="w-20 h-20 animate-pulse text-indigo-600" />
    </div>
  </div>
</template>
<script>
import { PhoneArrowUpRightIcon } from "@heroicons/vue/24/outline";
export default {
  name: "OfferReadyPage",
  props: [ 'offer_id' ],
  emits: [ 'processing', 'error', 'to' ],
  components: {
    PhoneArrowUpRightIcon,
  },
  methods: {
    on_error: function(message, error) {
      this.$emit('error', {error, message});
    }
  }
}
</script>
