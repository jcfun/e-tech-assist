<template>
  <view class="func-card">
    <view class="card-item">
      <u-icon name="order" color="#2979ff" size="55"></u-icon>
      我的帖子
    </view>
    <view class="card-item">
      <u-icon name="star" color="#ff9900" size="55"></u-icon>
      我的收藏
    </view>
    <view class="card-item">
      <u-icon name="eye" color="#51dc8b" size="55"></u-icon>
      浏览历史
    </view>
  </view>
</template>

<script setup lang="ts"></script>

<style scoped lang="scss">
  .func-card {
    display: flex;
    flex-direction: row;
    margin: 0 25rpx;
    margin-top: 25rpx;
    border-radius: 20rpx;
    background-color: #ffffff;
    overflow: hidden;

    .card-item {
      flex: 1;
      height: 150rpx;
      background-color: #ffffff;
      display: flex;
      justify-content: center;
      align-items: center;
      font-size: 30rpx;
    }
  }
</style>
