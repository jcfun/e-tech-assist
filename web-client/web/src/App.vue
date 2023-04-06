<template>
  <div>

  </div>
</template>

<script setup lang='ts'>
import { ref, reactive, onMounted } from 'vue';
import * as wasm from 'webassembly'
wasm.greet("wasm")


</script>

<style scoped lang='scss'></style>