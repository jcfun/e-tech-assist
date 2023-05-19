<style scoped lang="scss">
  .article-overview-box {
    margin-top: 30px;
    background-color: #fff;
    width: 100%;
    min-width: 1000px;
    border-radius: 5px;
    box-shadow: 0 0 5px #c9c9c9;
    .article-overview-list-wrap {
      .article-list-item {
        display: grid;
        grid-template-columns: 180px auto 20px;
        gap: 30px;
        align-items: center;
        margin: 30px;
        position: relative;
        .article-cover {
          display: flex;
          align-items: center;
          border-radius: 3px;
          overflow: hidden;
          img {
            width: 100%;
            object-fit: cover;
          }
        }
        .article-content {
          .article-title {
            font-size: 18px;
            font-weight: bold;
            margin-bottom: 10px;
            cursor: pointer;
            &:hover {
              color: #006fff;
              transition: all 0.2s ease-in;
            }
          }
          .article-status {
            display: flex;
            align-items: center;
            font-size: 15px;
            font-weight: 500;
            color: #717171;
            margin-bottom: 10px;
          }
          .article-info {
            display: flex;
            div {
              margin-right: 20px;
              color: #919191;
            }
          }
        }
        .article-operate {
          position: absolute;
          top: 3px;
          right: 0;
        }
      }
    }
  }
</style>

<template>
  <div class="article-overview-box">
    <div class="article-overview-list-wrap">
      <div class="article-list-item" v-for="(item, index) in state.articleList" :key="index">
        <div class="article-cover"><img :src="item.cover" alt="封面加载失败" /></div>
        <div class="article-content">
          <div class="article-title" @click="methods.toDetail(item)">{{ item.title }}</div>
          <div class="article-status">
            {{ item.createTime }}&nbsp;&nbsp;&nbsp;
            <a-tag :color="item.status == '0' ? 'green' : item.status == '1' ? 'red' : 'blue'">
              {{ item.status == '0' ? '草稿' : item.status == '1' ? '审核中' : '已发布' }}
            </a-tag>
          </div>
          <div class="article-info">
            <div><icon-eye />&nbsp;{{ item.viewCount }}</div>
            <div><icon-star />&nbsp;{{ item.likeCount }}</div>
            <div><icon-message />&nbsp;{{ item.commentCount }}</div>
          </div>
        </div>
        <div class="article-operate">
          <a-popover position="bottom" content-class="article-operate-popover-content">
            <icon-menu size="20" />
            <template #content>
              <div class="edit-btn" @click="methods.toEdit(item)">再次编辑</div>
              <div class="delete-btn" @click="$router.push('/create-center/overview')">删除文章</div>
            </template>
          </a-popover>
        </div>
      </div>
    </div>
    <scroll-to-top-button />
  </div>
</template>

<script setup lang="ts">
  import article from '@/api/modules/article';
  import type { QueryArticleVO } from '@/api/types/article';
  import { onMounted, onUnmounted } from 'vue';
  import { reactive } from 'vue';
  import ScrollToTopButton from '@/components/ScrollToTopButton.vue';
  import { useArticleDetailStore } from '@/stores/modules/article';
  const state = reactive({
    articleList: <Array<QueryArticleVO>>[],
    pageNo: 1,
    noMore: false,
    loading: false,
    scrollTop: 0, // 页面滚动条的高度
    windowHeight: 0, // 当前窗口的可视高度
    scrollHeight: 0, // 页面的总高度
  });
  const store = reactive({
    articleDetailStore: useArticleDetailStore(),
  });
  const methods = reactive({
    // 获取文章列表
    getArticleList: async () => {
      if (state.noMore || state.loading) {
        return;
      }
      state.loading = true;
      try {
        const res = await article.queryArticlesFq({
          byUserIdFlag: '1',
          pageNo: state.pageNo++,
          pageSize: 10,
        });
        if (res.code == 200) {
          if (res.data?.data.length < 10) {
            state.noMore = state.loading = true;
          }
          state.articleList = [...state.articleList, ...res.data.data];
        } else {
          state.pageNo--;
        }
      } catch {
        state.pageNo--;
      } finally {
        state.loading = false;
      }
    },
    // 获取滚动条当前的位置
    updateDimensions: () => {
      state.scrollTop = document.documentElement.scrollTop || document.body.scrollTop;
      state.windowHeight = document.documentElement.clientHeight || document.body.clientHeight;
      state.scrollHeight = document.documentElement.scrollHeight || document.body.scrollHeight;
    },
    // 滚动事件
    handleScroll: () => {
      methods.updateDimensions();
      if (state.scrollTop + state.windowHeight >= state.scrollHeight - 150) {
        methods.getArticleList();
      }
    },
    toDetail: (item: QueryArticleVO) => {
      store.articleDetailStore.setArticleDetail(item).then(() => {
        window.open(`/article/detail`, '_blank');
      });
    },
    toEdit: (item: QueryArticleVO) => {
      store.articleDetailStore.setArticleDetail(item).then(() => {
        window.open(`/create-center/article/edit`, '_blank');
      });
    },
    init: async function () {
      Promise.allSettled([this.getArticleList()]);
    },
  });
  methods.init();
  onMounted(() => {
    window.addEventListener('scroll', methods.handleScroll);
    methods.updateDimensions();
  });
  onUnmounted(() => {
    window.removeEventListener('scroll', methods.handleScroll);
  });
</script>
