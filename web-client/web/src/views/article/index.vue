<template>
  <div class="article-box">
    <div class="article-box-header">
      <a-carousel
        :style="{
          width: '100%',
          height: '350px',
          'margin-top': '50px',
          'border-radius': '5px',
          overflow: 'hidden',
        }"
        :auto-play="true"
        show-arrow="always"
        arrow-class="a-carousel-arrow-article"
        indicator-type="line"
      >
        <a-carousel-item v-for="(item, index) in articleCarouselList" :key="index">
          <img
            :src="item.cover"
            :style="{
              width: '100%',
              height: '100%',
              'object-fit': 'cover',
              cursor: 'pointer',
            }"
            @click="toDetail(item)"
          />
          <div
            style="
              position: absolute;
              bottom: 10px;
              padding: 0 30px;
              font-size: 20px;
              font-weight: bold;
              color: #fff;
              text-shadow: 0px 0px 10px rgba(0, 0, 0, 0.8);
              display: -webkit-box;
              -webkit-box-orient: vertical;
              -webkit-line-clamp: 1;
              display: -moz-box;
              -moz-box-orient: vertical;
              -moz-box-lines: 1;
              overflow: hidden;
            "
          >
            {{ item.title }}
          </div>
        </a-carousel-item>
      </a-carousel>
    </div>
    <div class="article-content-wrap">
      <div class="article-content">
        <div class="article-content-title">置顶</div>
        <div class="article-content-top">
          <div class="article-content-top-item" v-for="(item, index) in articleTopList" :key="index" @click="toDetail(item)">
            <img :src="item.cover" />
            <div class="mask">
              <h3 v-html="item.title"></h3>
              <p v-html="item.content"></p>
            </div>
          </div>
        </div>
        <div class="article-content-list">
          <div class="article-content-list-item" v-for="(item, index) in articleList" :key="index">
            <div class="article">
              <div class="title" @click="toDetail(item)">{{ item.title }}</div>
              <div class="content" v-html="item.content"></div>
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
      </div>
      <div class="article-side">
        <div class="article-side-function">
          <div class="search">
            <icon-search size="40" />
            <span>搜索文章</span>
          </div>
          <div class="pen" @click="$router.push('/create-center/article/publish')">
            <icon-pen-fill size="40" />
            <span>发布文章</span>
          </div>
          <div class="file">
            <icon-file size="40" />
            <span>我的文章</span>
          </div>
        </div>
        <div class="article-side-hot-wrap">
          <div class="article-side-hot-title">热门排行榜</div>
          <div class="article-side-hot-list">
            <div class="article-side-hot-item" v-for="(item, index) in articleHotList" :key="index" @click="toDetail(item)">
              {{ item.title }}
            </div>
          </div>
        </div>
      </div>
    </div>
    <scroll-to-top-button />
  </div>
</template>

<script setup lang="ts">
  import article from '@/api/modules/article';
  import ScrollToTopButton from '@/components/ScrollToTopButton.vue';
  import type { QueryArticleVO } from '@/api/types/article';
  import { ref } from 'vue';
  import dayjs from 'dayjs';
  import { timeInterval } from '@/utils';
  import { onMounted } from 'vue';
  import useRouter from '@/hooks/useRouter';
  import { useArticleDetailStore } from '@/stores/modules/article';
  import { onUnmounted } from 'vue';
  // --------------------变量区开始------------------------
  const router = useRouter();
  const articleDetailStore = useArticleDetailStore();
  const articleList = ref<Array<QueryArticleVO>>([]);
  const articleTopList = ref<Array<QueryArticleVO>>([]);
  const articleHotList = ref<Array<QueryArticleVO>>([]);
  const articleCarouselList = ref<Array<QueryArticleVO>>([]);
  let pageNo = 1;
  let noMore = false;
  let loading = false;
  let scrollTop = 0; // 页面滚动条的高度
  let windowHeight = 0; // 当前窗口的可视高度
  let scrollHeight = 0; // 页面的总高度
  // --------------------变量区结束------------------------

  // --------------------方法区开始------------------------
  // 获取文章列表
  const getArticlesFq = async () => {
    if (noMore || loading) {
      return;
    }
    loading = true;
    try {
      const res = await article.queryArticlesFq({
        pageNo: pageNo++,
        pageSize: 10,
      });
      if (res.code == 200) {
        if (res.data?.data.length < 10) {
          noMore = loading = true;
        }
        articleList.value = [...articleList.value, ...res.data.data];
      } else {
        pageNo--;
      }
    } catch {
      pageNo--;
    } finally {
      loading = false;
    }
  };
  // 获取滚动条当前的位置
  const updateDimensions = () => {
    scrollTop = document.documentElement.scrollTop || document.body.scrollTop;
    windowHeight = document.documentElement.clientHeight || document.body.clientHeight;
    scrollHeight = document.documentElement.scrollHeight || document.body.scrollHeight;
  };
  // 滚动事件
  const handleScroll = () => {
    updateDimensions();
    if (scrollTop + windowHeight >= scrollHeight - 150) {
      getArticlesFq();
    }
  };
  // 获取置顶文章
  const getTopArticles = async () => {
    const res = await article.queryTopArticles();
    if (res.code == 200) {
      articleTopList.value = res.data;
    }
  };
  // 获取热门文章
  const getHotArticles = async () => {
    const res = await article.queryHotArticles();
    if (res.code == 200) {
      articleHotList.value = res.data;
      articleCarouselList.value = res.data.slice(0, 5);
    }
  };
  Promise.all([getArticlesFq(), getTopArticles(), getHotArticles()]);
  const toDetail = (item: QueryArticleVO) => {
    articleDetailStore.setArticleDetail(item).then(() => {
      router.push('/article/detail');
    });
  };
  // --------------------方法区结束------------------------
  onMounted(() => {
    window.addEventListener('scroll', handleScroll);
    updateDimensions();
  });
  onUnmounted(() => {
    window.removeEventListener('scroll', handleScroll);
  });
</script>

<style scoped lang="scss">
  .article-box {
    width: 100%;
    max-width: 1200px;
    display: flex;
    flex-direction: column;
    .article-box-header {
      width: 100%;
    }
    .article-content-wrap {
      margin-top: 30px;
      width: 100%;
      display: flex;
      .article-content {
        width: 75%;
        display: flex;
        flex-direction: column;
        .article-content-title {
          color: #006fff;
          border-bottom: 1px solid #006fff;
          margin-bottom: 10px;
          font-size: 20px;
        }
        .article-content-top {
          display: grid;
          grid-template-columns: repeat(2, 1fr);
          grid-template-rows: minmax(0, 1fr);
          grid-gap: 10px;
          .article-content-top-item {
            max-height: 150px;
            width: 100%;
            border-radius: 5px;
            overflow: hidden;
            cursor: pointer;
            position: relative;
            img {
              width: 100%;
              height: 100%;
              object-fit: cover;
            }
            .mask {
              position: absolute;
              bottom: 0;
              left: 0;
              right: 0;
              height: 100%;
              padding: 0 5%;
              background-color: rgba(0, 0, 0, 0.25);
              color: #fff;
              transform: translateY(80%);
              transition: transform 0.3s ease-in-out, height 0.3s ease-in-out;
              h3 {
                margin-bottom: 5px;
              }
              p {
                display: -webkit-box;
                -webkit-box-orient: vertical;
                -webkit-line-clamp: 4;
                display: -moz-box;
                -moz-box-orient: vertical;
                -moz-box-lines: 4;
                overflow: hidden;
              }
            }
            &:hover .mask {
              transform: translateY(0%);
              background-color: rgba(0, 0, 0, 0.65);
            }
          }
        }
        .article-content-list {
          margin-top: 30px;
          .article-content-list-item {
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
      .article-side {
        width: 25%;
        height: 1000px;
        margin-left: 30px;
        .article-side-function {
          display: flex;
          justify-content: space-between;
          margin: 0 5px;
          .search,
          .pen,
          .file {
            display: flex;
            flex-direction: column;
            align-items: center;
            cursor: pointer;
            &:hover {
              color: #006fff;
              transition: all 0.2s ease-in;
            }
          }
        }
        .article-side-hot-wrap {
          margin-top: 50px;
          .article-side-hot-title {
            font-size: 20px;
            font-weight: 600;
            color: #006fff;
            margin-bottom: 10px;
          }
          .article-side-hot-list {
            margin-top: 10px;
            .article-side-hot-item {
              margin-bottom: 20px;
              font-size: 15px;
              font-weight: 500;
              cursor: pointer;
              display: -webkit-box;
              -webkit-box-orient: vertical;
              -webkit-line-clamp: 1;
              display: -moz-box;
              -moz-box-orient: vertical;
              -moz-box-lines: 1;
              overflow: hidden;
              &:hover {
                color: #006fff;
                transition: all 0.2s ease-in;
              }
            }
          }
        }
      }
    }
  }
</style>
