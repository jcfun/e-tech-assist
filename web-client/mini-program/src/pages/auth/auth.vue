<template>
  <view class="auth-box">
    <view class="welcome">欢迎使用 鄂助教</view>
    <view class="info">授权微信头像、昵称</view>
    <view class="icon">
      <u-image :showLoading="true" :src="src" mode="widthFix" width="100%" height="100%"></u-image>
    </view>
    <view class="info">为提供优质服务, 鄂助教需要获取你的以下信息:</view>
    <view class="list-item">
      <view class="dot"></view>
      <view class="list-text">你的公开信息(头像、昵称等)</view>
    </view>
    <view class="auth-btn">
      <u-button type="primary" text="授权进入鄂助教" shape="circle" @tap="toAuth"></u-button>
    </view>
  </view>
  <u-notify ref="authMsg"></u-notify>
</template>

<script setup lang='ts'>
import type { _NotifyRef } from '@ttou/uview-typings/types/notify';
import { ref } from 'vue';

const src = '/static/images/icon/icon.png';
const toAuth = () => {
  const authMsg = ref<_NotifyRef>(null);
  console.log(authMsg);
  uni.getUserProfile({
    desc: '获取你的昵称、头像、地区及性别',
    success: (res) => {
      authMsg.value.show({
        type: 'success',
        message: '授权成功',
        duration: 2000,
        fontSize: 20
      });
      console.log(res);
    },
    fail: (fail) => {
      authMsg.value.show({
        type: 'error',
        message: '授权失败',
        duration: 2000,
        fontSize: 20
      });
      console.log(fail);
    }
  });
};
</script>

<style scoped lang='scss'>
.auth-box {
  margin-left: 25rpx;

  .welcome {
    font-size: 50rpx;
    color: #000000;
    height: 50rpx;
    margin-top: 5%;
  }

  .info {
    font-size: 25rpx;
    color: #000000;
    height: 25rpx;
    margin-top: 5%;
  }

  .icon {
    margin-top: 10%;
    margin-left: -25rpx;
  }

  .list-item {
    display: flex;
    align-items: center;

    .dot {
      width: 5rpx;
      height: 5rpx;
      border-radius: 50%;
      background-color: black;
      margin-right: 10rpx;
      margin-top: 20rpx;
    }

    .list-text {
      font-size: 20rpx;
      margin-top: 20rpx;
    }
  }

  .auth-btn {
    margin-right: 25rpx;
    margin-top: 50rpx;
  }
}
</style>
