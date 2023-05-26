<style scoped lang="scss">
  .detail-box {
    background-color: #fff;
    padding: 20rpx;
    .article-detail {
      .article-detail-info {
        display: flex;
        align-items: center;
        margin-bottom: 20rpx;
        .article-info {
          margin-left: 20rpx;
          .creator,
          .create-time {
            font-size: 25rpx;
          }
        }
      }
      .article-detail-title {
        font-size: 32rpx;
        font-weight: 500;
        margin-bottom: 20rpx;
      }
      .divider {
        border-bottom: 1rpx solid #bdbdbd;
      }
      .article-detail-content {
        margin-top: 20rpx;
        margin-bottom: 30rpx;
        font-size: 28rpx;
        color: #2a2a2a;
        display: block;
        :deep(image) {
          margin: 15rpx 0;
          display: block;
        }
      }
    }
  }
</style>

<template>
  <view class="detail-box">
    <view class="article-detail">
      <view class="article-detail-info">
        <view class="avatar">
          <u-avatar shape="circle" size="80" :src="states.detail.avatar" customStyle="margin: -3px 5px -3px 0"></u-avatar>
        </view>
        <view class="article-info">
          <view class="creator">{{ states.detail.creator }}</view>
          <view class="create-time">
            {{ states.detail.createTime }}
          </view>
        </view>
      </view>
      <view class="article-detail-title">{{ states.detail.title }}</view>
      <view class="divider"></view>
      <view class="article-detail-content"><u-parse selectable copyLink :loadingImg="logo" :content="states.detail.content"></u-parse></view>
    </view>
  </view>
</template>

<script setup lang="ts">
  import type { QueryArticleVO } from '@/api/types/article';
  import { useArticleDetailStore } from '@/store/article';
  import logo from '@/static/images/logo/logo.png';
  import { reactive } from 'vue';
  import article from '@/api/modules/article';
  const states = reactive({
    detail: <QueryArticleVO>{},
  });
  const stores = reactive({
    articleDetailStore: useArticleDetailStore(),
  });
  const methods = {
    // 更新文章浏览量
    updateArticleViewCount: async () => {
      await article.updateArticleViewCount(states.detail.id);
    },
  };
  states.detail = stores.articleDetailStore.articleDetail;
  Promise.allSettled([methods.updateArticleViewCount()]);
  // states.detail.content = states.detail.content
  //   // .replace(/<img/gi, '<img style="width:100%; height:auto"; display: block; margin: 0 auto;')
  //   .replace(/<(\/)?pre[^>]*>/gi, '');
</script>
