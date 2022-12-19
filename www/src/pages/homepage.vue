<template>
  <div class="absolute inset-0 flex flex-col justify-center content-center gap-y-2">
    <h1 class="text-4xl font-bold tracking-tight text-gray-900 sm:text-5xl md:text-6xl text-center">
      <span class="block">WebRTC demo</span>
      <span class="block text-indigo-600 text-2xl sm:text-3xl md:text-4xl">Peer-2-peer text connection</span>
    </h1>
    <p class="mt-6 text-md md:text-lg leading-8 text-gray-600 text-center">
    An open-source project available on github
    </p>
    <div class="mt-1 text-center flex flex-row justify-center">
      <div>
        <input type="text" v-model="name" name="name" id="name" class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm" placeholder="Enter a name/id">
      </div>
    </div>
    <div class="mt-1 flex flex-col sm:flex-row gap-y-2 gap-x-2 justify-center">
      <div class="flex flex-row justify-center">
        <ButtonComponent @click="initiate" :disabled="name.length === 0">Initiate a Connection</ButtonComponent>
      </div>
      <div class="flex flex-row justify-center">
        <ButtonComponent @click="$emit('to', 'offer');" :disabled="name.length === 0">Have an Offer</ButtonComponent>
      </div>
    </div>
  </div>
</template>
<script>
import ButtonComponent from "@/components/buttoncomponent.vue";
export default {
  name: "HomePage",
  props: ['modelValue'],
  emits: [ 'processing', 'error', 'to', 'start', 'update:modelValue' ],
  components: {
    ButtonComponent,
  },
  computed: {
    name: {
      get() { return this.modelValue; },
      set(v) { this.$emit('update:modelValue', v); }
    },
  },
  methods: {
    initiate: function() {
      try {
console.log("initiate");
this.$emit('start');
      } catch (e) {
        this.on_error("Error during connection initiatlisation", e);
      }
    },
    on_error: function(message, error) {
      this.$emit('error', {error, message});
    }
  }
}
</script>
