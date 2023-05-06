<template>
  <view class="reply-box" @click="toReply">
    <u-icon name="chat" color="#4b99ff" size="50"></u-icon>
    <span class="description">回复</span>
  </view>
  <view class="u-page">
    <view class="msg-box" v-for="(item, index) in children" :key="index">
      <view class="msg-title">{{ item.title }}</view>
      <view class="msg-profile">
        <view class="avatar">
          <u-avatar :src="item.senderAvatar" size="80" shape="circle"></u-avatar>
        </view>
        <view class="profile">
          <view class="send-recipient">
            <view class="send">{{ item.senderEmail }}</view>
            <u-icon name="arrow-rightward" size="25"></u-icon>
            <view class="recipient">{{ item.recipientEmail }}</view>
          </view>
          <view class="time">{{ item.createTime }}</view>
        </view>
      </view>
      <view class="msg-content">
        <u-text user-select size="28" color="#000" :text="item.content"></u-text>
      </view>
    </view>
    <view class="msg-box">
      <view class="msg-title">{{ quickMsgDetail.title }}</view>
      <view class="msg-profile">
        <view class="avatar">
          <u-avatar :src="quickMsgDetail.senderAvatar" size="80" shape="circle"></u-avatar>
        </view>
        <view class="profile">
          <view class="send-recipient">
            <view class="send">{{ quickMsgDetail.senderEmail }}</view>
            <u-icon name="arrow-rightward" size="25"></u-icon>
            <view class="recipient">{{ quickMsgDetail.recipientEmail }}</view>
          </view>
          <view class="time">{{ quickMsgDetail.createTime }}</view>
        </view>
      </view>
      <view class="msg-content">
        <u-text user-select size="28" color="#000" :text="quickMsgDetail.content"></u-text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
  import quickMsg from '@/api/modules/quickMsg';
  import type { UpdateReadFlagDTO } from '@/api/types/quickMsg';
  import { useQuickMsgDetailStore } from '@/store/quick-msg';
  import { onShow } from '@dcloudio/uni-app';
  import { ref } from 'vue';
  import { useUserStore } from '@/store/user';
  const user = useUserStore().userInfo;
  // 获取回复的快捷消息并更新
  onShow(() => {
    quickMsg.getQuickMsgReplyList(quickMsgDetail.value.id).then(res => {
      children.value = quickMsgDetail.value.children = res.data.data;
    });
  });
  const quickMsgDetailStore = useQuickMsgDetailStore();
  const quickMsgDetail = ref(quickMsgDetailStore.getQuickMsgDetail);
  const children = ref(quickMsgDetail.value.children);
  const toReply = () => {
    uni.navigateTo({
      url: '/pages/quickMsg/send?prev=detail',
    });
  };
  // 如果当前用户不是发送者，则将消息标记为已读
  if (user.email != quickMsgDetail.value.senderEmail) {
    console.log('update!');
    const ids: Array<string> = [];
    ids.push(quickMsgDetail.value.id);
    quickMsgDetail.value.children.forEach(item => {
      ids.push(item.id);
    });
    console.log('ids ====> ', ids);
    quickMsg.updateReadFlag({ ids, readFlag: '1' } as UpdateReadFlagDTO);
  }
</script>

<style scoped lang="scss">
  .reply-box {
    z-index: 999;
    position: fixed;
    right: 50rpx;
    bottom: 150rpx;
    background-color: #ffffff;
    border-radius: 50%;
    width: 100rpx;
    height: 100rpx;
    box-shadow: 10rpx 10rpx 15rpx #cacaca;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    &:hover {
      box-shadow: 10rpx 10rpx 20rpx #c2c2c2;
    }
    .description {
      font-size: 18rpx;
      color: #4b99ff;
    }
  }
  .u-page {
    .msg-box {
      background-color: #ffffff;
      padding: 20rpx 0;
      margin: 15rpx 0;
      .msg-title {
        margin: 0 0 0 35rpx;
      }
      .msg-profile {
        display: flex;
        direction: row;
        align-items: center;
        margin: 30rpx 30rpx 0 30rpx;
        .avatar {
          width: 80rpx;
          height: 80rpx;
          display: flex;
          justify-content: flex-start;
          align-items: center;
          :deep(.u-avatar) {
            background-color: #ffffff !important;
          }
        }
        .profile {
          margin-left: 20rpx;
          .send-recipient {
            display: flex;
            .send {
              margin-right: 10rpx;
              font-size: 25rpx;
            }
            .recipient {
              margin-left: 10rpx;
              font-size: 25rpx;
            }
          }
          .time {
            margin-top: 10rpx;
            font-size: 23rpx;
            color: #747474;
          }
        }
      }
      .msg-content {
        margin: 50rpx 35rpx 0 35rpx;
      }
    }
  }
</style>
