<template>
  <div class="fixed inset-0 bg-gray-500 bg-opacity-75"></div>
  <div class="fixed inset-0 z-10 overflow-y-auto">
    <div class="flex min-h-full items-end justify-center items-center">
      <div class="relative rounded-md bg-red-50 p-4">
        <div class="flex">
          <div class="flex-shrink-0">
            <XCircleIcon class="h-5 w-5 text-red-400" />
          </div>
          <div class="ml-3 md:min-w-[32rem]">
            <h3 class="text-sm font-medium text-red-800">
              <slot></slot>
            </h3>
            <div class="mt-2 text-sm text-red-700" v-if="error !== null">
              <ul role="list" class="list-disc pl-5 space-y-1">
              <li>
              Error: {{ error }}
              </li>
              <li>
              Stack: {{ error.stack }}
              </li>
              </ul>
            </div>
            <div class="mt-4 flex flex-row-reverse">
              <div class="mr-3">
                <button type="button" 
                        class="rounded-md bg-red-50 px-2 py-1.5 text-sm font-medium text-red-800 hover:bg-red-100 focus:outline-none focus:ring-2 focus:ring-red-600 focus:ring-offset-2 focus:ring-offset-red-50"
                        @click="$emit('dismiss')">
                  Dismiss
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
<script>
import { XCircleIcon } from "@heroicons/vue/24/solid";
export default {
  name: "ErrorComponent",
  emits: [ 'dismiss' ],
  components: {
    XCircleIcon,
  },
  props: ['error', 'message']
};
</script>
