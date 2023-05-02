<template>
  <view class="user-login">
    <view class="login-area" @click="toAuth">
      <span class="avatar">
        <u-icon v-if="!token.token" name="account-fill" color="#FFFFF8" size="100"></u-icon>
        <u-avatar v-else :src="imgSrc" :size="120" mode="aspectFill"></u-avatar>
      </span>
      <view class="user-info">
        <span class="nickname">{{ nickname }}</span>
        <span class="account">账号: {{ account }}</span>
      </view>
    </view>
    <view class="qrcode" @click="toMore">
      <u-image :showLoading="true" :src="qrcode" width="20px" height="20px"></u-image>
      <span class="arrow"><u-icon name="arrow-right" color="#2979ff" size="20"></u-icon></span>
    </view>
    <!-- <button class="account-btn" plain @click="toMore">更多信息</button> -->
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
  let account = ref('');
  account.value = userInfo?.account ?? '';
  // 头像
  const imgSrc = ref(userInfo.avatarUrl);
  // 二维码
  const qrcode = '/static/images/mine/qrcode.png';

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
      align-items: stretch;
      padding-right: 25rpx;
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
      .user-info {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        align-items: stretch;
        flex-grow: 1;
        margin: 40rpx 20rpx;
        .nickname {
          font-size: 40rpx;
        }
        .account {
          color: #383838;
          font-size: 20rpx;
        }
      }
    }

    .qrcode {
      width: 100%;
      min-height: 100%;
      font-size: 20rpx;
      position: absolute;
      right: 25rpx;
      padding: 0px 6px;
      height: 45rpx;
      display: flex;
      align-items: center;
      justify-content: flex-end;
      color: black;
      .arrow {
        margin-left: 30rpx;
      }
    }
  }
</style>
