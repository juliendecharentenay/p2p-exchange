<template>
  <div class="absolute inset-0 flex flex-col">
    <div class="mt-12 flex flex-row justify-between text-center text-gray-400">
        <div class="ml-12">(other)</div>
        <div class="mr-12">{{ name }} (you)</div>
    </div>
    <div class="ml-8 mr-8 grow flex-col gap-y-1 text-gray-600">
      <div v-for="(m,index) in messages"
           :key="index">
        <div class="flex row"
             :class="{'justify-start': m.from === 'peer', 'justify-end': m.from === 'me'}">
          {{ m.message }}
        </div>
      </div> 
    </div>
    <div class="mb-12 ml-12 mr-12 flex flex-row justify-end">
      <div class="w-full md:max-w-sm lg:max-w-md">
        <input type="text" v-model="message" name="message" id="message"
               class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
               placeholder="Type your message and press enter" 
               @change="send();"
               />
      </div>
    </div>
  </div>
<!--
    <div class="flex flex-col content-center">
    <p class="mt-6 text-lg leading-8 text-gray-600 text-center">
    Connected...
    </p>
    <div class="mt-4 flex flex-row gap-x-2 justify-center">
      blabla
    </div>
  </div>
-->
</template>
<script>
export default {
  name: "OfferPage",
  emits: [ 'processing', 'error', 'to', 'send' ],
  props: [ 'name', 'messages' ],
  data: function() {
    return {
      message: '',
    };
  },
  components: {
  },
  methods: {
    send: function() {
      try {
console.log("Send ", this.message);
        this.$emit('send', this.message);
        this.message = '';
      } catch (e) {
        this.on_error("Error sending message", e);
      }
    },
    on_error: function(message, error) {
      this.$emit('error', {error, message});
    }
  }
}
</script>
