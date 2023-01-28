<template>
  <div class="absolute inset-0 flex flex-col">
    <div class="mt-12 flex flex-row justify-between text-center text-gray-400">
        <div class="ml-12">(other)</div>
        <div class="mr-12">{{ name }} (you)</div>
    </div>
    <div class="pl-8 pr-8 pb-2 grow flex flex-col-reverse overflow-y-auto gap-y-1 text-gray-600 items-stretch">
      <div v-for="(m,index) in messages"
           :key="index"
           class="flex row"
           :class="{'justify-start': m.from === 'peer', 'justify-end': m.from === 'me'}">
          {{ m.message }}
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
