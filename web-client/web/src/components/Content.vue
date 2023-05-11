<template>
  <div class="content">
    <router-view v-slot="{ Component, route }">
      <transition
        :enter-to-class="`animate__animated ${route.meta.transitionEnter}`"
        :leave-from-class="`animate__animated ${route.meta.transitionLeave}`"
      >
        <component :is="Component" />
      </transition>
    </router-view>
  </div>
</template>

<style scoped lang="scss">
  .content {
    display: flex;
    flex-grow: 1;
    min-height: 600px;
    position: relative;
  }
</style>
