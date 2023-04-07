<template>
  <view class="user-login">
    <view class="login-area" @click="toAuth">
      <span class="avatar">
        <u-icon name="account" color="#FFFFF8" size="100"></u-icon>
      </span>
      <span class="login-btn">立即登录</span>
    </view>
    <button class="account-btn" plain>账户信息</button>
  </view>
</template>

<script setup lang='ts'>
const toAuth = () => {
  uni.navigateTo({
    url: '/pages/auth/auth'
  })
}
</script>

<style scoped lang='scss'>
.user-login {
  display: flex;
  flex-direction: row;
  align-items: center;
  position: relative;

  .login-area {
    display: flex;
    flex-direction: row;
    align-items: center;
    padding-right: 25rpx;
  }

  .avatar {
    background-color: rgb(194, 226, 255);
    border-radius: 100%;
    width: 120rpx;
    height: 120rpx;
    display: flex;
    justify-content: center;
    align-items: center;
    margin: 25rpx;
  }

  .login-btn {
    font-size: 40rpx;
    margin-left: 20rpx;
  }

  .account-btn {
    font-size: 20rpx;
    position: absolute;
    right: 25rpx;
    border-radius: 20rpx;
    border: 1rpx solid #c4c4c4;
    padding: 0px 6px;
    height: 45rpx;
    display: flex;
    align-items: center;
    color: black;
  }
}
</style>
