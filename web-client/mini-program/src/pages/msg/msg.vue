<template>
  <view class="msg-box">
    <view class="func-area">
      <view class="func-item">
        <view class="priv">
          <u-icon name="email-fill" color="#8bd7ff" size="60"></u-icon>
        </view>
        私信
      </view>
      <view class="func-item">
        <view class="reply">
          <u-icon name="chat-fill" color="#cda5ff" size="60"></u-icon>
        </view>
        回复
      </view>
      <view class="func-item">
        <view class="like">
          <u-icon name="thumb-up-fill" color="#ffc559" size="60"></u-icon>
        </view>
        点赞
      </view>
      <view class="func-item" @click="toList">
        <view class="quick-msg">
          <u-icon name="bell-fill" color="#92c1ff" size="60"></u-icon>
        </view>
        快捷消息
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
  const toList = () => {
    uni.navigateTo({
      url: '/pages/quick-msg/list',
    });
  };
</script>

<style scoped lang="scss">
  .msg-box {
    .func-area {
      display: flex;
      flex-direction: row;
      justify-content: space-around;
      margin-top: 30rpx;
      .func-item {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        font-size: 25rpx;
        view {
          height: 80rpx;
          width: 80rpx;
          border-radius: 30rpx;
          display: flex;
          align-items: center;
          justify-content: center;
          margin-bottom: 10rpx;
        }
        .priv {
          background-color: #d4f8ffd8;
        }
        .reply {
          background-color: #f4e6ffd8;
        }
        .like {
          background-color: #feedccd8;
        }
        .quick-msg {
          background-color: #d1e5ffd8;
        }
      }
    }
  }
</style>
