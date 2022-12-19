<template>
  <div class="absolute inset-0 flex flex-col justify-center content-center gap-y-2">
    <p class="mt-6 text-lg leading-8 text-gray-600 sm:text-center">
    Offer:
    </p>
    <div class="mt-4 flex flex-row gap-x-2 justify-center">
      <div v-if="code.length < 8">
        <label for="offer" class="sr-only">Offer code</label>
        <input v-model="code" type="offer" name="offer" id="offer" class="block w-full p-2 rounded-md border border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm" placeholder="Enter the offer code">
      </div>
      <div v-else>
        <PhoneArrowDownLeftIcon class="w-20 h-20 animate-pulse text-indigo-600" />
      </div>
    </div>
  </div>
</template>
<script>
import { PhoneArrowDownLeftIcon } from "@heroicons/vue/24/outline";
export default {
  name: "OfferPage",
  emits: [ 'processing', 'error', 'to', 'start' ],
  components: {
    PhoneArrowDownLeftIcon,
  },
  data: function() {
    return {
      p_code: "",
    }
  },
  computed: {
    code: {
      get()  { return this.p_code; },
      set(v) { this.p_code = v; if (this.p_code.length === 4) { this.$emit('start', {id: this.p_code}); } },
    }
  },
  methods: {
    on_error: function(message, error) {
      this.$emit('error', {error, message});
    }
  }
}
</script>
