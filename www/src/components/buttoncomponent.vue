<template>
  <button type="button"
          class="inline-flex items-center rounded-md border border-transparent px-6 py-3 text-base font-medium text-white shadow-sm"
          :class="{'bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2': ! disabled,
                   'bg-indigo-300': disabled }"
          :disabled="disabled"
           >
    <slot></slot>
  </button>
</template>
<script>
export default {
  name: "ButtonComponent",
  props: [ 'disabled' ],
}
</script>
