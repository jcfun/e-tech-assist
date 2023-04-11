<template>
  <view class="more-page">
    <view class="list-item" @click="toAccount">
      <span class="title">账户设置</span>
      <span class="arrow"><u-icon name="arrow-right" color="#c4c4c4" size="25"></u-icon></span>
    </view>
    <view class="list-item">
      <span class="title">问题反馈</span>
      <span class="arrow"><u-icon name="arrow-right" color="#c4c4c4" size="25"></u-icon></span>
    </view>
    <view class="list-item last-item">
      <span class="title">关于我们</span>
      <span class="arrow"><u-icon name="arrow-right" color="#c4c4c4" size="25"></u-icon></span>
    </view>
  </view>
  <view class="logout-btn" @click="logout">
    <span class="content">退出登录</span>
  </view>
</template>

<script setup lang="ts">
  // import { ref } from 'vue';
  import type { Token, UserInfo } from '@/models/login';
  import { useUserStore } from '@/store/user';
  const user = useUserStore();
  const logout = () => {
    uni.showModal({
      title: '提示',
      content: '确定退出登录吗？',
      success: res => {
        if (res.confirm) {
          user.token = {} as Token;
          user.userInfo = {} as UserInfo;
          uni.reLaunch({
            url: '/pages/mine/mine',
          });
        }
      },
    });
  };

  const toAccount = () => {
    uni.navigateTo({
      url: '/pages/account/account',
    });
  };
</script>

<style scoped lang="scss">
  .more-page {
    background-color: white;
    margin-top: 30rpx;
    // box-sizing: border-box;
    // border: 1px solid transparent;
    .list-item {
      display: flex;
      // margin-bottom: 30rpx;
      // border-bottom: 1rpx solid #e5e5e5;
      display: flex;
      align-items: center;
      height: 100rpx;
      .title {
        font-size: 30rpx;
        margin-left: 30rpx;
      }
      .arrow {
        margin-left: auto;
        text-align: center;
        margin-right: 30rpx;
        width: 50rpx;
      }
    }
  }
  .logout-btn {
    height: 100rpx;
    background-color: white;
    margin-top: 30rpx;
    display: flex;
    align-items: center;
    justify-content: center;
    .content {
      font-size: 30rpx;
    }
  }
</style>
