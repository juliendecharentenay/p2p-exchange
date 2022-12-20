<template>
  <div class="bg-white inset-0">
    <HomePage v-if="page === 'home'" 
         v-model="name"
         @processing="(e) => {processing = e;}"
         @error="(e) => {error = e;}"
         @to="(e) => {page = e;}" 
         @start="() => {start_offer();}" />

    <!-- Handling responding to an offer and making an answer -->
    <OfferPage v-if="page === 'offer'"
         @processing="(e) => {processing = e;}"
         @error="(e) => {error = e;}"
         @to="(e) => {page = e;}" 
         @start="(evt) => {start_answer(evt.id);}" />
    <WaitAnswerAcceptPage v-if="page === 'wait_answer_accept'"
         @processing="(e) => {processing = e;}"
         @error="(e) => {error = e;}"
         @to="(e) => {page = e;}" />

    <!-- Handling making offer and accepting answer -->
    <OfferReadyPage v-if="page === 'offer_ready'"
         :offer_id="get_connection_id()"
         @processing="(e) => {processing = e;}"
         @error="(e) => {error = e;}"
         @to="(e) => {page = e;}" />
    <AcceptAnswerPage v-if="page === 'accept_answer'"
         @processing="(e) => {processing = e;}"
         @error="(e) => {error = e;}"
         @to="(e) => {page = e;}" />

    <!-- P2P connection established -->
    <ConnectedPage v-if="page === 'connected'"
         :messages="messages"
         :name="name"
         @processing="(e) => {processing = e;}"
         @error="(e) => {error = e;}"
         @to="(e) => {page = e;}" 
         @send="send"
         />

    <div class="absolute top-1 left-1" v-if="page !== 'home'">
      <a href="#" @click="page = 'home'"
         class="font-medium text-indigo-600 hover:text-indigo-500"
         >Home</a>
    </div>
    <div class="absolute bottom-1 left-1 text-xs text-gray-600">
      Copyright (C) 2022, Julien de Charentenay
    </div>

    <ErrorComponent
      :error="error.error"
      @dismiss="error = null;"
      v-if="error !== null">
      {{ error.message }}
    </ErrorComponent>
    <ProcessingComponent
     :message="processing"
     v-if="processing !== null" />
  </div>
</template>

<script>
import AcceptAnswerPage from "@/pages/acceptanswerpage.vue";
import ConnectedPage from "@/pages/connectedpage.vue";
import HomePage from "@/pages/homepage.vue";
import OfferPage from "@/pages/offerpage.vue";
import OfferReadyPage from "@/pages/offerreadypage.vue";
import WaitAnswerAcceptPage from "@/pages/waitansweracceptpage.vue";
import ErrorComponent from "@/components/errorcomponent.vue";
import ProcessingComponent from "@/components/processingcomponent.vue";

import { Connection } from "@/lib/connection.js";

export default {
  name: 'App',
  components: {
    AcceptAnswerPage,
    ConnectedPage,
    HomePage,
    OfferPage,
    OfferReadyPage,
    WaitAnswerAcceptPage,
    ErrorComponent,
    ProcessingComponent,
  },
  data: function() {
    return {
      page: 'home',
      error: null,
      processing: null,
      connection: null,
      name: '',
      messages: [],
    };
  },
  computed: {
  },
  methods: {
    get_connection_id: function() {
      return this.connection.get_id();
    },
    start_answer: function(id) {
      this.start({ 
           name: this.name, id,
           on_error: (e) => {this.on_error("Error in connection", e);},
           on_ready: ()  => {this.page = 'wait_answer_accept';},
           on_connected: () => {this.page = 'connected';},
           on_message: this.on_message,
      });
    },
    start_offer: function() {
      this.start({
           name: this.name,
           on_error: (e) => {this.on_error("Error in connection", e);},
           on_ready: ()  => {this.page = 'offer_ready';},
           on_connected: () => {this.page = 'connected';},
           on_message: this.on_message,
      });
    },
    start: function(config) {
      try {
        this.connection = new Connection(config);
        this.connection.start();
      } catch (e) {
        this.on_error("Error when starting connection", e);
      }
    },
    stop: function() {
      try {
        this.connection.close();
      } catch (e) {
        this.on_error("Error when stoping connection", e);
      }
    },
    on_message: function(message) {
      try {
        this.messages.push({from: 'peer', message});
      } catch(e) {
        this.on_error("Error in on_message", e);
      }
    },
    send: function(message) {
      try {
        this.connection.send(message);
        this.messages.push({from: 'me', message});
      } catch (e) {
        this.on_error("Error when sending message", e);
      }
    },
    on_error: function(message, error) {
      if (this.error === null) {
        console.error("error:",message, error);
        this.error = { error, message };
      }
    }
  }
}
</script>
