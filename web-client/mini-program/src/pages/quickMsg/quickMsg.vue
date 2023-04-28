<template>
  <view class="send-box">
    <view class="send-btn">
      <u-button type="primary" shape="circle" size="mini" text="发送" @click="send"></u-button>
    </view>
  </view>
  <view class="sender-box">
    <span class="avatar">
      <u-avatar :src="user?.userInfo.avatarUrl" :size="80" mode="aspectFill"></u-avatar>
    </span>
    <view class="userinfo">
      <view>{{ user?.userInfo.account }}</view>
      <view>{{ user?.userInfo.phoneNumber }}</view>
    </view>
  </view>
  <u-line></u-line>
  <view class="recipient-box">
    <span class="sign">发送至</span>
    <view class="userinfo">
      <span class="avatar">
        <u-avatar :src="user?.userInfo.avatarUrl" :size="40" mode="aspectFill"></u-avatar>
      </span>
      <view style="margin-left: 15rpx" v-if="user?.userInfo.account">{{ user?.userInfo.account }}</view>
    </view></view
  >
  <u-line></u-line>
  <view class="title-box"><span style="margin-right: 30rpx">标题</span><u-input v-model="title" border="none" clearable></u-input></view>
  <u-line></u-line>
  <view class="content">
    <u-textarea v-model="content" count maxlength="2000" autoHeight border="none"></u-textarea>
  </view>
</template>

<script setup lang="ts">
  import { ref } from 'vue';
  import { useUserStore } from '@/store/user';
  import quickMsg from '@/api/modules/quickMsg';
  import type { CreateQuickMsgDTO } from '@/api/types/quickMsg';
  const user = useUserStore();
  if (!user.userInfo.account) {
    uni.redirectTo({
      url: '/pages/auth/auth?prev=quickMsg',
    });
  }
  const content = ref('');
  const title = ref('');
  const send = () => {
    const dto = {
      senderId: user.userInfo.id,
      recipientId: user.userInfo.id,
      title: title.value,
      content: content.value,
      sendType: '1',
    } as CreateQuickMsgDTO;
    quickMsg.sendQuickMsg(dto).then(res => {
      if (res.code == '200') {
        uni.reLaunch({
          url: '/pages/mine/mine',
          success: () => {
            uni.showToast({
              title: res.msg,
              duration: 3000,
              icon: 'none',
            });
          },
        });
      } else {
        uni.showToast({
          title: res.msg,
          duration: 3000,
          icon: 'none',
        });
      }
    });
  };
</script>

<style scoped lang="scss">
  .send-box {
    display: flex;
    justify-content: flex-end;
    margin: 10rpx 20rpx 10rpx 0;
    .send-btn {
      width: 100rpx;
    }
  }
  .sender-box {
    margin-left: 20rpx;
    margin-bottom: 25rpx;
    display: flex;
    flex-direction: row;
    align-items: center;
    .userinfo {
      margin-left: 30rpx;
      text-align: left;
    }
  }
  .recipient-box {
    display: flex;
    flex-direction: row;
    align-items: center;
    margin: 20rpx 0 20rpx 20rpx;
    .sign {
      margin-right: 30rpx;
    }
    .userinfo {
      display: flex;
      flex-direction: row;
      align-items: center;
      padding: 5rpx;
      border: solid 1px #e5e5e5;
      border-radius: 25rpx;
    }
  }

  .title-box {
    margin: 20rpx 0 20rpx 20rpx;
    display: flex;
    flex-direction: row;
    align-items: center;
  }

  .content {
    :deep(.u-textarea) {
      background-color: transparent !important;
      min-height: 500rpx;
      .u-textarea__field {
        min-height: inherit !important;
      }
      .u-textarea__count {
        background-color: transparent !important;
      }
    }
  }
</style>
