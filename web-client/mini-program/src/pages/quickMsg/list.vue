<template>
  <view class="new-box" @click="toNew">
    <u-icon name="edit-pen-fill" color="#4b99ff" size="60"></u-icon>
  </view>
  <view class="u-page">
    <u-list @scrolltolower="scrolltolower">
      <u-list-item v-for="(item, index) in indexList" :key="index">
        <u-cell :title="item.senderEmail" center isLink @click="toDetail(item)">
          <template #icon>
            <u-badge
              :isDot="true"
              :show="(item.readFlag == '0' || item.children.filter(item => item.readFlag != '1').length != 0) && item.senderEmail != user.email"
              type="error"
            ></u-badge>
            <u-avatar shape="circle" size="100" :src="item.senderAvatar" customStyle="margin: -3px 5px -3px 0"></u-avatar>
          </template>
          <template #label>
            <view class="cell-label">
              <view class="cell-title-content">
                <u-text user-select size="25" color="#000" :lines="1" :text="item.title"></u-text>
                <u-text user-select size="20" color="#3c3c3c" :lines="2" :text="`${item.content.replace('\n', '')}`"></u-text>
                <!-- .substring(0, 20)}${item.content.length > 20 ? '...' : '' -->
              </view>
              <view class="cell-time">
                <u-text user-select size="20" class="label-time" :text="item.createTime"></u-text>
              </view>
            </view>
          </template>
        </u-cell>
      </u-list-item>
      <u-loadmore :status="loadmoreStatus" loadingIcon="semicircle" line fontSize="25" />
    </u-list>
  </view>
  <u-loading-page :loading="loading" loading-mode="semicircle" font-size="40" icon-size="65" loading-text="loading..."></u-loading-page>
</template>

<script setup lang="ts">
  import quickMsg from '@/api/modules/quickMsg';
  import type { QueryQuickMsgVO } from '@/api/types/quickMsg';
  import { onShow } from '@dcloudio/uni-app';
  import { ref } from 'vue';
  import { useQuickMsgDetailStore } from '@/store/quick-msg';
  import { useUserStore } from '@/store/user';
  const user = useUserStore().userInfo;
  const indexList = ref([] as QueryQuickMsgVO[]);
  let pageNo = 1;
  let noMore = false;
  const loadmoreStatus = ref('loadmore');
  const loading = ref(true);
  const scrolltolower = () => {
    loadmore();
  };
  const toNew = () => {
    uni.navigateTo({
      url: '/pages/quickMsg/send?prev=list',
    });
  };
  const loadmore = () => {
    if (noMore) {
      loadmoreStatus.value = 'nomore';
      return;
    }
    loadmoreStatus.value = 'loading';
    quickMsg
      .getQuickMsgList(pageNo++, 15)
      .then(res => {
        if (res.data?.data.length < 15) {
          noMore = true;
        }
        indexList.value = [...indexList.value, ...res.data.data];
      })
      .finally(() => {
        loading.value = false;
        loadmoreStatus.value = 'loadmore';
      });
  };
  loadmore();
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const toDetail = (item: QueryQuickMsgVO) => {
    useQuickMsgDetailStore().setQuickMsgDetail(item);
    uni.navigateTo({
      url: '/pages/quickMsg/detail',
    });
  };
  onShow(() => {
    pageNo = 1;
    noMore = false;
    indexList.value = [];
    loading.value = true;
    loadmore();
  });
</script>

<style scoped lang="scss">
  .new-box {
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
    justify-content: center;
    align-items: center;
    &:hover {
      box-shadow: 10rpx 10rpx 20rpx #c2c2c2;
    }
  }

  .u-page {
    .cell-label {
      display: flex;
      justify-content: space-between;
      .cell-title-content {
        display: flex;
        flex-direction: column;
        width: 350rpx;
      }
      .cell-time {
        display: flex;
        :deep(.u-text) {
          align-self: flex-end;
        }
      }
    }
  }
</style>
