<template>
  <router-view v-slot="{ Component, route }">
    <keep-alive :include="cacheRoutes.cachedRoutes">
      <component :is="Component" :key="route.fullPath" />
    </keep-alive>
  </router-view>
</template>

<script lang="ts">
  import useAppConfigStore from '@/store/modules/config';
  import useCachedRouteStore from '@/store/modules/cache';
  import { defineComponent } from 'vue';
  export default defineComponent({
    name: 'Main',
    setup() {
      const appStore = useAppConfigStore();
      const cacheRoutes = useCachedRouteStore();
      return {
        appStore,
        cacheRoutes,
      };
    },
  });
</script>

<style>
  .h-full-vh {
    height: 100vh;
  }
</style>
