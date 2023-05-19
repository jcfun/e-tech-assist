<style scoped lang="scss">
  .article-detail-box {
    width: 100%;
    background-color: #f5f6f7;
    display: flex;
    justify-content: center;
    .article-detail-wrap {
      display: flex;
      width: 100%;
      max-width: 1500px;
      margin: 30px 0;
      .article-detail {
        width: 75%;
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
          // font-size: 15px;
          margin-top: 30px;
          :deep(img) {
            width: 100%;
          }
        }
      }
      .article-extend {
        width: 25%;
        margin-left: 20px;
        .article-hot-list {
          padding: 20px 15px;
          background-color: #fff;
          border-radius: 5px;
          min-height: 300px;
          .article-hot-title {
            font-size: 20px;
            font-weight: bold;
            margin-bottom: 10px;
          }
          .divider {
            border-bottom: 1px solid #bdbdbd;
            margin-bottom: 10px;
          }
          .article-hot-item {
            margin-top: 20px;
            .article-hot-item-title {
              font-size: 15px;
              font-weight: 500;
              cursor: pointer;
              &:hover {
                color: #006fff;
                transition: all 0.2s ease-in;
              }
            }
            .article-hot-item-date {
              margin-top: 5px;
              color: #939393;
            }
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
          <div class="avatar"><img :src="state.articleCountAndAvatar.avatar" /></div>
          <div>{{ state.detail.creator }}</div>
          <div>{{ `文章数：${state.articleCountAndAvatar.totalArticleCount}` }}</div>
          <div>
            {{ timeInterval(state.detail.createTime, dayjs()) }}
          </div>
          <div>{{ `阅读量：${state.detail.viewCount}` }}</div>
          <div>{{ `点赞数：${state.detail.likeCount}` }}</div>
          <div>{{ `评论数：${state.detail.commentCount}` }}</div>
        </div>
        <div class="divider"></div>
        <div class="article-detail-content" v-html="state.detail.content"></div>
      </div>
      <div class="article-extend">
        <div class="article-hot-list">
          <div class="article-hot-title">更多好文</div>
          <div class="divider"></div>
          <div class="article-hot-item" v-for="(item, index) in state.hotArticles" :key="index">
            <div class="article-hot-item-title" @click="methods.toDetail(item)">{{ item.title }}</div>
            <div class="article-hot-item-date">{{ item.createTime }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { onMounted, reactive } from 'vue';
  import type { QueryArticleVO, QueryUserArticleVO } from '@/api/types/article';
  import { useArticleDetailStore } from '@/stores/modules/article';
  import { useUserStore } from '@/stores/modules/user';
  import dayjs from 'dayjs';
  import { timeInterval } from '@/utils';
  import article from '@/api/modules/article';
  import Prism from 'prismjs';
  import { onUpdated } from 'vue';
  import useRouter from '@/hooks/useRouter';

  const store = reactive({
    articleDetailStore: useArticleDetailStore(),
    userStore: useUserStore(),
  });
  const state = reactive({
    detail: <QueryArticleVO>{},
    articleCountAndAvatar: <QueryUserArticleVO>{},
    hotArticles: <QueryArticleVO[]>[],
  });
  const router = useRouter();
  const methods = reactive({
    // 获取用户文章数量和头像
    getUserArticleCountAndAvatar: async () => {
      const res = await article.queryUserArticleCountAndAvatar(state?.detail?.creatorId);
      if (res.code == 200) {
        state.articleCountAndAvatar = res.data;
      }
    },
    // 获取热门文章
    getHotArticles: async () => {
      const res = await article.queryHotArticles();
      if (res.code == 200) {
        state.hotArticles = res.data;
      }
    },
    toDetail: (item: QueryArticleVO) => {
      store.articleDetailStore.setArticleDetail(item).then(() => {
        const currentRoute = router.currentRoute;
        const fullPath = currentRoute.value.fullPath;
        window.open(fullPath, '_blank');
        // router.go(0);
      });
    },
    init: async function () {
      Promise.allSettled([this.getUserArticleCountAndAvatar(), this.getHotArticles()]);
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
