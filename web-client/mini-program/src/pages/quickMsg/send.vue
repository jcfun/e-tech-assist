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
        <u-avatar :src="recipient.avatarUrl" :size="40" mode="aspectFill"></u-avatar>
      </span>
      <view style="margin-left: 15rpx" v-if="recipient.identity != ''">{{ recipient.identity }}</view>
      <view style="margin-left: 15rpx; width: 400rpx" v-else><u-input v-model="recipientIdentity" border="none" clearable></u-input></view>
    </view>
  </view>
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
  import { onLoad } from '@dcloudio/uni-app';
  import { isEmpty } from '@/utils';
  import type { Ref } from 'vue';
  import { useQuickMsgDetailStore } from '@/store/quick-msg';
  const quickMsgDetail = useQuickMsgDetailStore().getQuickMsgDetail;
  // 上一个页面
  let prev = '';
  onLoad(options => {
    prev = options?.prev ?? '';
    recipient.value = isEmpty(options?.recipient)
      ? { identity: '', avatarUrl: '/static/images/logo/logo.png' }
      : { identity: options?.recipient.identity, avatarUrl: options?.recipient.avatarUrl };
    if (prev == 'detail') {
      recipient.value =
        user.userInfo.email == quickMsgDetail.senderEmail
          ? { identity: quickMsgDetail.recipientEmail, avatarUrl: quickMsgDetail.recipientAvatar }
          : { identity: quickMsgDetail.senderEmail, avatarUrl: quickMsgDetail.senderAvatar };
    }
  });
  const user = useUserStore();
  if (!user.userInfo.account) {
    uni.redirectTo({
      url: '/pages/auth/auth?prev=send',
    });
  }
  const recipientIdentity = ref('');
  const content = ref('');
  const title = ref('');
  const recipient: Ref<{
    identity: string;
    avatarUrl: string;
  }> = ref({
    identity: '',
    avatarUrl: '/static/images/logo/logo.png',
  });
  const send = () => {
    const dto = {
      senderId: user.userInfo.id,
      recipientIdentity: recipient.value.identity == '' ? recipientIdentity.value : recipient.value.identity,
      title: title.value,
      content: content.value,
      sendMethod: '1',
      msgType: prev == 'detail' ? '1' : '0',
      replyId: prev == 'detail' ? quickMsgDetail.id : null,
    } as CreateQuickMsgDTO;
    quickMsg.sendQuickMsg(dto).then(res => {
      if (res.code == 200) {
        if (prev == 'list' || prev == 'detail') {
          uni.navigateBack({
            delta: 1,
            success: () => {
              uni.showToast({
                title: res.msg,
                duration: 3000,
                icon: 'none',
              });
            },
          });
        } else {
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
        }
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
