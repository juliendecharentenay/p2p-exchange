<template>
  <div class="absolute inset-0 flex flex-col justify-center content-center gap-y-2">
    <p class="mt-6 text-lg leading-8 text-gray-600 sm:text-center">
    Enter the offer code:
    </p>
    <div class="mt-4 flex flex-row gap-x-2 justify-center">
      blabla
    </div>
  </div>
</template>
<script>
export default {
  name: "OfferPage",
  emits: [ 'processing', 'error', 'to' ],
  components: {
  },
  methods: {
    on_error: function(message, error) {
      this.$emit('error', {error, message});
    }
  }
}
</script>
