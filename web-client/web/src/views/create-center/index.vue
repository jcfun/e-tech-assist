<template>
  <div class="create-center-box">
    <sidebar></sidebar>
    <router-view></router-view>
  </div>
</template>

<script setup lang="ts">
  import sidebar from './components/SideBar.vue';
</script>

<style scoped lang="scss">
  .create-center-box {
    background-color: #f9f9f9;
    width: 100%;
    min-width: 1500px;
    padding: 0 20%;
    display: flex;
    justify-content: center;
  }
</style>
