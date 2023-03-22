<template>
  <div>

  </div>
</template>

<script setup lang='ts'>
import { ref, reactive, onMounted } from 'vue';
import * as wasm from 'rust-wasm';
// onMounted(async () => {
//   wasm.default("/node_modules/rust-wasm")
// })

wasm.greet()


</script>

<style scoped lang='scss'></style>