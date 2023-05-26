<style scoped lang="scss">
  .article-detail-box {
    width: 100%;
    background-color: #fff;
    display: flex;
    justify-content: center;
    .article-detail-wrap {
      display: flex;
      justify-content: center;
      width: 80%;
      min-width: 1200px;
      margin: 30px 0;
      .article-detail {
        padding: 30px 40px;
        background-color: #fff;
        border-radius: 5px;
        .article-detail-title {
          font-size: 30px;
          font-weight: 500;
          margin-bottom: 10px;
        }
        .article-detail-info {
          display: flex;
          align-items: center;
          margin-bottom: 20px;
          div {
            font-size: 15px;
            color: #8e8e8e;
            margin-right: 20px;
          }
          .avatar {
            width: 40px;
            height: 40px;
            img {
              width: 100%;
              border-radius: 5px;
              object-fit: cover;
            }
          }
        }
        .divider {
          border-bottom: 1px solid #bdbdbd;
        }
        .article-detail-content {
          margin-top: 30px;
          :deep(img) {
            width: 100%;
          }
        }
      }
    }
  }
</style>

<template>
  <div class="article-detail-box">
    <div class="article-detail-wrap">
      <div class="article-detail">
        <div class="article-detail-title">{{ state.detail.title }}</div>
        <div class="article-detail-info">
          <div class="avatar"><img :src="state.detail.avatar" /></div>
          <div>{{ state.detail.creator }}</div>
          <div>
            {{ timeInterval(state.detail.createTime, dayjs()) }}
          </div>
          <div>{{ `文章数：${state.articleCount}` }}</div>
          <div>{{ `阅读量：${state.detail.viewCount}` }}</div>
          <div>{{ `点赞数：${state.detail.likeCount}` }}</div>
          <div>{{ `评论数：${state.detail.commentCount}` }}</div>
        </div>
        <div class="divider"></div>
        <div class="article-detail-content" v-html="state.detail.content"></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { onMounted, reactive } from 'vue';
  import type { QueryArticleVO } from '@/api/types/article';
  import { useArticleDetailStore } from '@/store/modules/article';
  import useUserStore from '@/store/modules/user';
  import dayjs from 'dayjs';
  import { timeInterval } from '@/utils';
  import article from '@/api//requests/article';
  import Prism from 'prismjs';
  import { onUpdated } from 'vue';

  const store = reactive({
    articleDetailStore: useArticleDetailStore(),
    userStore: useUserStore(),
  });
  const state = reactive({
    detail: {} as QueryArticleVO,
    articleCount: 0,
  });
  const methods = reactive({
    // 获取用户文章数量和头像
    getUserArticleCount: async () => {
      const res = await article.queryUserArticleCount(state?.detail?.creatorId);
      if (res.code == 200) {
        state.articleCount = res.data;
      }
    },
    init: async function () {
      Promise.allSettled([this.getUserArticleCount()]);
    },
  });
  onMounted(() => {
    window.scrollTo({ top: 0 });
    state.detail = store.articleDetailStore.getArticleDetail;
    methods.init();
    Prism.highlightAll();
  });
  onUpdated(() => {
    Prism.highlightAll();
  });
</script>
