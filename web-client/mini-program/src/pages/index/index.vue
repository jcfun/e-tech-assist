<style scoped lang="scss">
  .index-box {
    .index-header {
      background-color: #fff;
      .index-search {
        margin: 10rpx 30rpx;
        padding-top: 20px;
      }
      .index-tab {
        margin: 0 15rpx;
        margin-bottom: 20rpx;
      }
    }
    .index-content {
      .index-info-list {
        margin: 0 15rpx;
        .index-info-item {
          background-color: #fff;
          border-radius: 20rpx;
          margin: 20rpx 0;
          padding: 30rpx;
          .index-info-item-author {
            display: flex;
            align-items: center;
            .index-info-item-avatar {
              height: 60rpx;
              margin-right: 20rpx;
              display: flex;
              align-items: center;
            }
            .index-info-item-nickname {
              font-size: 30rpx;
              font-weight: 500;
            }
          }
          .index-info-item-title {
            margin-top: 20rpx;
            font-size: 30rpx;
          }
          .index-info-item-content {
            font-size: 20rpx;
            color: #575757;
          }
          .index-info-item-extend {
            height: 40rpx;
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin: 0 50rpx;
            margin-top: 20rpx;
            .extend-item {
              display: flex;
              align-items: center;
              font-size: 27rpx;
              color: #5d5d5d;
            }
          }
        }
      }
    }
  }
</style>

<template>
  <view class="index-box">
    <view class="index-header">
      <view class="index-search">
        <u-search
          v-if="!states.loading"
          height="70"
          :show-action="true"
          actionText="搜索"
          :animation="true"
          searchIconSize="40"
          v-model="states.queryDTO.title"
          @search="methods.onSearch"
          @custom="methods.onSearch"
        ></u-search>
      </view>
      <view class="index-tab">
        <u-tabs
          :list="states.tabItems"
          :current="states.currentTab"
          :activeStyle="{ color: '#5286ff', transform: 'scale(1.1)', transition: 'transform 0.3s ease' }"
          lineColor="transparent"
          @change="methods.onSwitchTab"
        ></u-tabs>
      </view>
    </view>
    <view class="index-content">
      <view class="index-info-list">
        <view class="index-info-item" v-for="(item, index) in states.indexList" :key="index" @click="methods.toDetail(item)">
          <view class="index-info-item-author">
            <view class="index-info-item-avatar">
              <u-avatar shape="circle" size="60" :src="item.avatar" customStyle="margin: -3px 5px -3px 0"></u-avatar>
            </view>
            <view class="index-info-item-nickname">{{ item.creator }}</view>
          </view>
          <view class="index-info-item-title">{{ item.title }}</view>
          <view class="index-info-item-content">
            <u-parse selectable :content="methods.extractImages(item.content)"></u-parse>
          </view>
          <view class="index-info-item-extend">
            <div class="extend-item">
              <u-icon name="eye" size="35"></u-icon>
              &nbsp;&nbsp;{{ item.viewCount }}
            </div>
            <div class="extend-item">
              <u-icon name="star" size="35"></u-icon>
              &nbsp;&nbsp;{{ item.collectCount }}
            </div>
            <div class="extend-item">
              <u-icon name="thumb-up" size="35"></u-icon>
              &nbsp;&nbsp;{{ item.likeCount }}
            </div>
          </view>
        </view>
        <u-loadmore style="margin-bottom: 10rpx" :status="states.loadmoreStatus" loadingIcon="semicircle" line fontSize="25" />
      </view>
    </view>
  </view>
  <u-loading-page :loading="states.loading" loading-mode="semicircle" font-size="40" icon-size="65" loading-text="loading..."></u-loading-page>
</template>

<script setup lang="ts">
  import { reactive } from 'vue';
  import article from '@/api/modules/article';
  import type { QueryArticleDTO, QueryArticleVO } from '@/api/types/article';
  import { onReachBottom } from '@dcloudio/uni-app';
  import { useArticleDetailStore } from '@/store/article';

  const states = reactive({
    tabItems: [
      {
        name: '全部',
      },
      {
        name: '推荐',
      },
      {
        name: '置顶',
      },
      {
        name: '资讯',
      },
    ],
    currentTab: 1,
    noMore: false,
    loading: true,
    loadmoreStatus: 'loadmore',
    pageNo: 1,
    indexList: <Array<QueryArticleVO>>[],
    queryDTO: <QueryArticleDTO>{
      pageSize: 10,
      title: '',
      topFlag: '',
      hotFlag: '',
    },
  });
  const stores = reactive({
    articleDetailStore: useArticleDetailStore(),
  });
  const methods = {
    getArticlesFq: () => {
      if (states.noMore) {
        states.loadmoreStatus = 'nomore';
        states.loading = false;
        return;
      }
      states.loadmoreStatus = 'loading';
      article
        .getArticlesFq({
          ...states.queryDTO,
          pageNo: states.pageNo++,
        })
        .then(res => {
          if (res.code == 200) {
            if (res.data?.data.length < 10) {
              states.noMore = true;
            }
            console.log('res ===> ', res.data.data);
            states.indexList = [...states.indexList, ...res.data.data];
          } else {
            states.pageNo--;
          }
        })
        .catch(_err => {
          states.pageNo--;
        })
        .finally(() => {
          states.loading = false;
          states.loadmoreStatus = 'loadmore';
        });
    },
    extractImages: (content: string) => {
      let imgReg = /<img.*?(?:>|\/>)/gi;
      let srcReg = /src=['"]?([^'"]*)['"]?/i;
      let arr = content.match(imgReg);
      let srcArr = arr?.map(item => {
        let src = item.match(srcReg);
        // 图片裁剪显示
        return `<div
                  style="
                  background-image: url('${src?.[1]}');
                  width: 100%;
                  height: 200rpx;
                  background-position: center;
                  background-repeat: no-repeat;
                  background-size: cover;"
                >
                </div>`;
      });
      if (srcArr) {
        return `<div style='border-radius: 10rpx; overflow: hidden; margin-top: 20rpx; display: grid; grid-template-columns: repeat(auto-fit, minmax(calc(33.33% - 5px), 1fr)); grid-gap: 5px;'>${
          srcArr?.slice(0, 3).join('') || ''
        }</div>`;
      }
    },
    getArticles: () => {
      switch (states.currentTab) {
        case 0:
          methods.getArticlesFq();
          break;
        case 1: {
          states.queryDTO.hotFlag = '1';
          methods.getArticlesFq();
          break;
        }
        case 2: {
          states.queryDTO.topFlag = '1';
          methods.getArticlesFq();
          break;
        }
        case 3:
          break;
      }
    },
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    onSwitchTab: (e: any) => {
      states.pageNo = 1;
      states.noMore = false;
      states.indexList = [];
      states.loadmoreStatus = 'loadmore';
      states.queryDTO.topFlag = '';
      states.queryDTO.hotFlag = '';
      states.currentTab = e.index;
      methods.getArticles();
    },
    onSearch: () => {
      states.loading = true;
      states.pageNo = 1;
      states.noMore = false;
      states.indexList = [];
      states.loadmoreStatus = 'loadmore';
      methods.getArticles();
    },
    toDetail: (item: QueryArticleVO) => {
      stores.articleDetailStore.setQuickMsgDetail(item).then(() => {
        uni.navigateTo({
          url: '/pages/index/detail',
        });
      });
    },
    init: function () {
      Promise.allSettled([this.getArticles()]);
    },
  };
  methods.init();
  onReachBottom(() => {
    methods.init();
  });
</script>
