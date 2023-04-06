<template>
  <view>
    <mine-login v-if="user.token"></mine-login>
    <mine-userinfo v-else></mine-userinfo>
  </view>
</template>

<script setup lang='ts'>
import { useUserStore } from '../../store/user'

const user = useUserStore()

</script>

<style scoped lang='scss'></style>
