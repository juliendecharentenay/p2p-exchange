<template>
  <div class="max-w-xl mx-auto mt-24">
    <div class="rounded-md bg-green-50 p-4">
      <div class="flex">
        <div class="flex-shrink-0">
          <CheckCircle class="h-5 w-5 text-green-400" />
        </div>
        <div class="ml-3">
          <h3 class="text-sm font-medium text-green-800">
            Processing in process
          </h3>
          <p class="mt-2 text-sm text-green-700">
            {{ message }}
          </p>
        </div>
      </div>
    </div>
  </div>
</template>
<script>
export default {
  name: "ProcessingComponent",
  props: [ 'message' ],
};
</script>
