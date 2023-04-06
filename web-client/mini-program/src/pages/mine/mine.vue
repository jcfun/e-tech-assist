<template>
  <view>
    <mine-login></mine-login>
    <mine-userinfo></mine-userinfo>
    <u-text>{{ User.token }}</u-text>
  </view>
</template>

<script setup lang='ts'>
import { useUserStore } from '../../store/user'

const User = useUserStore()

</script>

<style scoped lang='scss'></style>
