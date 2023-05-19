<template>
  <div @click="scrollToTop" v-show="showButton" class="scroll-to-top-button">
    <a-popover position="left" content-class="scroll-to-top-popover-content">
      <i style="font-size: 20px" class="iconfont icon-rocket"></i>
      <template #content>
        <div>回到顶部</div>
      </template>
    </a-popover>
  </div>
</template>
<script setup lang="ts">
  import { ref, onMounted, onBeforeUnmount } from 'vue';
  import '@/assets/iconfont/iconfont.css';
  const showButton = ref(false);

  const handleScroll = () => {
    showButton.value = window.scrollY > 500;
  };

  const scrollToTop = () => {
    window.scrollTo({
      top: 0,
      behavior: 'smooth',
    });
  };

  onMounted(() => {
    window.addEventListener('scroll', handleScroll);
  });

  onBeforeUnmount(() => {
    window.removeEventListener('scroll', handleScroll);
  });
</script>
<style scoped lang="scss">
  .scroll-to-top-button {
    position: fixed;
    height: 40px;
    width: 40px;
    bottom: 100px;
    right: 80px;
    z-index: 99;
    padding: 5px;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: #006fff;
    color: #fff;
    cursor: pointer;
  }
</style>
