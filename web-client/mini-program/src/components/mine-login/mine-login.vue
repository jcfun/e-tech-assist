<template>
  <view class="user-login">
    <view class="login-area" @click="toAuth">
      <span class="avatar">
        <u-icon v-if="!token.token" name="account-fill" color="#FFFFF8" size="100"></u-icon>
        <u-avatar v-else :src="imgSrc" :size="120" mode="aspectFill"></u-avatar>
      </span>
      <span class="nickname">{{ nickname }}</span>
    </view>
    <button class="account-btn" plain @click="toMore">更多信息</button>
  </view>
</template>

<script setup lang="ts">
  import { useUserStore } from '@/store/user';
  import { onShow } from '@dcloudio/uni-app';
  import { ref } from 'vue';
  const user = useUserStore();
  let token = user.token;
  let userInfo = user.userInfo;
  // 昵称
  let nickname = ref('立即登录');
  nickname.value = userInfo?.nickname ?? '立即登录';
  // 头像
  const imgSrc = ref(userInfo.avatarUrl);

  const toAuth = () => {
    if (token.token) return;
    uni.navigateTo({
      url: '/pages/auth/auth',
    });
  };

  const toMore = () => {
    if (token.token) {
      uni.navigateTo({
        url: '/pages/more/more',
      });
    } else {
      uni.navigateTo({
        url: '/pages/auth/auth',
      });
    }
  };

  onShow(() => {
    token = user.token;
    userInfo = user.userInfo;
    nickname.value = userInfo?.nickname ?? '立即登录';
  });
</script>

<style scoped lang="scss">
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

    .nickname {
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
