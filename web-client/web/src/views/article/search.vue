<style scoped lang="scss">
  .search-box {
    width: 100%;
    max-width: 1350px;
    margin-top: 50px;
    .search-header {
      margin: 0 10%;
      display: flex;
      align-items: center;
      justify-content: center;
      button {
        margin-left: 20px;
        width: 80px;
      }
    }
    .search-content {
      margin-top: 50px;
      .search-content-item {
        border-top: 1px solid #e8e8e8;
        padding: 10px 0;
        display: flex;
        border-bottom: 1px solid #eaeaea;
        align-items: center;
        .article {
          width: 80%;
          .title {
            font-size: 18px;
            font-weight: 600;
            // line-height: 100%;
            color: #3c3c3c;
            display: -webkit-box;
            -webkit-box-orient: vertical;
            -webkit-line-clamp: 1;
            display: -moz-box;
            -moz-box-orient: vertical;
            -moz-box-lines: 1;
            overflow: hidden;
            cursor: pointer;
            &:hover {
              color: #006fff;
              transition: all 0.3s ease-in;
            }
          }
          .content {
            // margin-top: 10px;
            margin-bottom: 5px;
            font-size: 12px;
            display: -webkit-box;
            -webkit-box-orient: vertical;
            -webkit-line-clamp: 2;
            display: -moz-box;
            -moz-box-orient: vertical;
            -moz-box-lines: 2;
            overflow: hidden;
          }
          div {
            color: #868686;
          }
          .info {
            display: flex;
            div {
              display: flex;
              align-items: center;
              min-width: 60px;
              font-size: 12px;
              margin-right: 10px;
            }
          }
        }
        .cover {
          display: flex;
          align-items: center;
          border-radius: 3px;
          overflow: hidden;
          width: 20%;
          img {
            width: 100%;
            object-fit: cover;
          }
        }
      }
    }
  }
</style>

<template>
  <div class="search-box">
    <div class="search-header">
      <a-input v-model="state.searchValue" size="large" placeholder="请输入搜索内容" allow-clear />
      <a-button type="primary" :loading="state.loading" @click="methods.onSearch">搜索</a-button>
    </div>
    <div class="search-content">
      <div class="search-content-item" v-for="(item, index) in state.articleList" :key="index">
        <div class="article">
          <div class="title" @click="methods.toDetail(item)">{{ item.title }}</div>
          <div class="content" v-html="item.content.replace(/<img.*?>/g, '')"></div>
          <div class="info">
            <div>{{ item.creator }}</div>
            <div>
              {{ timeInterval(item.createTime, dayjs()) }}
            </div>
            <div><icon-eye />&nbsp;{{ item.viewCount }}</div>
            <div><icon-star />&nbsp;{{ item.likeCount }}</div>
            <div><icon-message />&nbsp;{{ item.commentCount }}</div>
          </div>
        </div>
        <div class="cover">
          <img :src="item.cover" />
        </div>
      </div>
    </div>
    <scroll-to-top-button />
  </div>
</template>

<script setup lang="ts">
  import { onMounted, reactive } from 'vue';
  import type { QueryArticleVO } from '@/api/types/article';
  import articles from '@/api/modules/article';
  import { useArticleDetailStore } from '@/stores/modules/article';
  import useRouter from '@/hooks/useRouter';
  import { timeInterval } from '@/utils';
  import dayjs from 'dayjs';
  import ScrollToTopButton from '@/components/ScrollToTopButton.vue';
  const state = reactive({
    searchValue: '',
    articleList: <Array<QueryArticleVO>>[],
    pageNo: 1,
    noMore: false,
    loading: false,
    scrollTop: 0,
    windowHeight: 0,
    scrollHeight: 0,
    router: useRouter(),
  });
  const stores = reactive({
    articleDetailStore: useArticleDetailStore(),
  });
  const methods = reactive({
    onSearch: async () => {
      state.pageNo = 1;
      state.noMore = false;
      state.articleList = [];
      await methods.getArticlesFq();
    },
    getArticlesFq: async () => {
      if (state.noMore || state.loading) {
        return;
      }
      state.loading = true;
      try {
        const res = await articles.queryArticlesFq({
          pageNo: state.pageNo++,
          pageSize: 10,
          title: state.searchValue,
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
        methods.getArticlesFq();
      }
    },
    toDetail: (item: QueryArticleVO) => {
      stores.articleDetailStore.setArticleDetail(item).then(() => {
        state.router.push('/article/detail');
      });
    },
    init: function () {
      Promise.allSettled([this.getArticlesFq()]);
    },
  });
  onMounted(() => {
    window.addEventListener('scroll', methods.handleScroll);
    methods.updateDimensions();
    methods.init();
  });
</script>
