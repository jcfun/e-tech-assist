<style scoped lang="scss">
  .home-box {
    width: 100%;
    margin-top: 20px;
    display: flex;
    flex-direction: column;
    align-items: center;
    .carousel {
      width: 100%;
      :deep(.carousel__pagination) {
        position: relative;
        margin-top: -20px;
        z-index: 999;
      }
    }
    .carousel__item {
      position: relative;
      height: 350px;
      width: 100%;
      background-color: #fff;
      color: var(--vc-clr-white);
      font-size: 20px;
      border-radius: 8px;
      display: flex;
      justify-content: center;
      align-items: center;
      margin: 0 15px;
      border-radius: 10px;
      overflow: hidden;
      margin-bottom: 50px;
      box-shadow: 0 20px 25px #d1d1d1;
      box-sizing: border-box;
      .carousel__item__img {
        width: 100%;
        height: 100%;
        object-fit: cover;
      }
      .carousel-title {
        position: absolute;
        font-size: 30px;
        text-shadow: 0px 0px 10px rgba(0, 0, 0, 0.8);
        display: -webkit-box;
        -webkit-box-orient: vertical;
        -webkit-line-clamp: 1;
        display: -moz-box;
        -moz-box-orient: vertical;
        -moz-box-lines: 1;
        overflow: hidden;
      }
      cursor: pointer;
    }
    .article-choiceness {
      margin-top: 30px;
      width: 1350px;
      .article-choiceness-title {
        display: flex;
        justify-content: space-between;
        .title {
          font-size: 25px;
          font-weight: bold;
        }
        .more {
          height: 25px;
          border: 1px solid #bdbdbd;
          border-radius: 5px;
          padding: 5px 8px;
          padding-left: 16px;
          display: flex;
          align-items: center;
          font-size: 13px;
          font-weight: 400;
          cursor: pointer;
          color: #5f5f5f;
          &:hover {
            background-color: #f2f6f9;
            transition: all 0.2s ease-in;
          }
        }
      }
      .article-choiceness-content {
        display: flex;
        .article-item {
          width: 20%;
          margin-right: 15px;
          cursor: pointer;
          // 最后一个元素取消右边距
          &:last-child {
            margin-right: 0;
          }
          &:hover .article-title {
            color: #006fff;
            transition: all 0.2s ease-in;
          }
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
          .article-title {
            font-size: 16px;
            font-weight: bold;
            margin-top: 10px;
            display: -webkit-box;
            -webkit-box-orient: vertical;
            -webkit-line-clamp: 1;
            display: -moz-box;
            -moz-box-orient: vertical;
            -moz-box-lines: 1;
            overflow: hidden;
          }
        }
      }
    }
    .information-express {
      margin-top: 80px;
      width: 1350px;
      .information-express-title {
        display: flex;
        justify-content: space-between;
        .title {
          font-size: 25px;
          font-weight: bold;
        }
        .more {
          height: 25px;
          border: 1px solid #bdbdbd;
          border-radius: 5px;
          padding: 5px 8px;
          padding-left: 16px;
          display: flex;
          align-items: center;
          font-size: 13px;
          font-weight: 400;
          cursor: pointer;
          color: #5f5f5f;
          &:hover {
            background-color: #f2f6f9;
            transition: all 0.2s ease-in;
          }
        }
      }
    }
  }
</style>

<template>
  <div class="home-box">
    <Carousel ref="myCarousel" :items-to-show="2.5" :autoplay="3000" :wrap-around="true" v-model="states.currentSlide">
      <Slide v-for="(item, index) in states.articleChoicenessList" :key="index">
        <div class="carousel__item" @click="methods.slideTo(item, index)">
          <img class="carousel__item__img" :src="item.cover" alt="图片加载失败" />
          <div class="carousel-title">{{ item.title }}</div>
        </div>
      </Slide>
      <template #addons>
        <Pagination />
      </template>
    </Carousel>
    <div class="article-choiceness">
      <div class="article-choiceness-title">
        <div class="title">文章精选</div>
        <div class="more" @click="$router.push('/article')">更多<icon-right /></div>
      </div>
      <div class="article-choiceness-content">
        <div class="article-item" v-for="(item, index) in states.articleChoicenessList" :key="index" @click="methods.toDetail(item)">
          <div class="article-cover">
            <img :src="item.cover" alt="图片加载失败" />
          </div>
          <div class="article-title">{{ item.title }}</div>
        </div>
      </div>
    </div>
    <div class="information-express">
      <div class="information-express-title">
        <div class="title">资讯速递</div>
        <div class="more" @click="$router.push('/article')">更多<icon-right /></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import articles from '@/api/modules/article';
  import type { QueryArticleVO } from '@/api/types/article';
  import { useArticleDetailStore } from '@/stores/modules/article';
  import { reactive } from 'vue';
  import { ref } from 'vue';
  import { Carousel, Pagination, Slide } from 'vue3-carousel';
  import 'vue3-carousel/dist/carousel.css';
  const myCarousel = ref();
  const states = reactive({
    currentSlide: 0,
    articleChoicenessList: <Array<QueryArticleVO>>[],
  });
  const store = reactive({
    articleDetailStore: useArticleDetailStore(),
  });
  const methods = {
    slideTo: (item: QueryArticleVO, index: number) => {
      if (index == states.articleChoicenessList.length - 1 && states.currentSlide == 0) {
        myCarousel.value.prev();
      } else if (index == 0 && states.currentSlide == states.articleChoicenessList.length - 1) {
        myCarousel.value.next();
      } else if (index < states.currentSlide) {
        myCarousel.value.prev();
      } else if (index > states.currentSlide) {
        myCarousel.value.next();
      } else {
        methods.toDetail(item);
      }
    },
    toDetail: (item: QueryArticleVO) => {
      store.articleDetailStore.setArticleDetail(item).then(() => {
        window.open(`/article/detail`, '_blank');
      });
    },
    // 获取热门文章
    getHotArticles: async () => {
      const res = await articles.queryHotArticles();
      if (res.code == 200) {
        states.articleChoicenessList = res.data.slice(0, 5);
      }
    },
    init: function () {
      Promise.allSettled([this.getHotArticles()]);
    },
  };
  methods.init();
</script>
