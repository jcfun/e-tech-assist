<style scoped lang="scss">
  .overview-box {
    width: 100%;
    background-color: #f9f9f9;
    margin: 30px 0;
    min-width: 1000px;
    z-index: 0;
    .overview-table-wrap {
      border-radius: 5px;
      border-top: 4px solid #006fff;
      box-shadow: 0 0 5px #c9c9c9;
      overflow: hidden;
      .overview-table-title {
        font-size: 18px;
        font-weight: bold;
        margin: 10px 0;
        margin-left: 30px;
      }
    }
    .overview-tip-wrap {
      margin-top: 80px;
      height: 500px;
      box-shadow: 0 0 5px #c9c9c9;
      background-color: #fff;
      position: relative;
      padding: 20px;
      .overview-tip-title {
        font-size: 18px;
        font-weight: bold;
        display: flex;
        align-items: center;
      }
      .divider {
        margin-top: 5px;
        margin-bottom: 10px;
        border-bottom: 1px solid #bdbdbd;
      }
      .overview-tip-content {
        font-size: 16px;
      }
      &::before {
        content: '';
        display: block;
        position: absolute;
        width: 100%;
        height: 500px;
        transform: rotate(-1.2deg);
        top: 0;
        left: 0;
        z-index: -2;
        box-shadow: 0 0 5px #c9c9c9;
        background-color: #fff;
      }
      &::after {
        content: '';
        display: block;
        position: absolute;
        width: 100%;
        height: 500px;
        transform: rotate(1.2deg);
        top: 0;
        left: 0;
        z-index: -1;
        box-shadow: 0 0 5px #c9c9c9;
        background-color: #fff;
      }
    }
  }
</style>

<template>
  <div class="overview-box">
    <div class="overview-table-wrap">
      <div class="overview-table-title">我的创作数据</div>
      <a-table :data="state.data" :pagination="false">
        <template #columns>
          <a-table-column
            :cell-style="{ height: '50px', 'font-size': '16px' }"
            v-for="(item, index) in state.columns"
            :key="index"
            :title="item.title"
            :data-index="item.dataIndex"
            align="center"
          >
          </a-table-column>
        </template>
      </a-table>
    </div>
    <div class="overview-tip-wrap">
      <div class="overview-tip-title"><icon-bulb size="25" />&nbsp;创作小tips</div>
      <div class="divider"></div>
      <div class="overview-tip-content" v-html="state.tipContent"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import articles from '@/api/modules/article';
  import { reactive } from 'vue';
  const state = reactive({
    columns: [
      {
        title: '类型',
        dataIndex: 'type',
      },
      {
        title: '总创作数',
        dataIndex: 'totalArticleCount',
      },
      {
        title: '总阅读量',
        dataIndex: 'totalViewCount',
      },
      {
        title: '总点赞数',
        dataIndex: 'totalLikeCount',
      },
      {
        title: '总评论数',
        dataIndex: 'totalCommentCount',
      },
      {
        title: '总收藏数',
        dataIndex: 'totalCollectCount',
      },
      {
        title: '总转发数',
        dataIndex: 'totalForwardCount',
      },
    ],
    data: reactive([
      {
        type: '文章',
        totalArticleCount: 0,
        totalViewCount: 0,
        totalLikeCount: 0,
        totalCommentCount: 0,
        totalCollectCount: 0,
        totalForwardCount: 0,
      },
      {
        type: '资讯',
        totalArticleCount: 0,
        totalViewCount: 0,
        totalLikeCount: 0,
        totalCommentCount: 0,
        totalCollectCount: 0,
        totalForwardCount: 0,
      },
    ]),
    tipContent: `<p><br></p><p style="text-align: start;">如果你已经在第三方平台发布了文章, 想要简单的搬到鄂助教来。那么可以直接 <code>ctrl + c/v</code> 进行复制。 鄂助教的富文本编辑器支持将主流文章平台的内容直接复制过来并保持格式不变动， 如知乎、B站专栏、微信公众号等。<br>但对于一些根本没格式 &nbsp;的网站不支持, 这些网站一复制过来文字就都堆在一起了，只能手动改，无解。</p><p style="text-align: start;">需要注意的是, 要保持格式那么复制时需要在第三方平台的<strong>编辑页</strong>复制, 直接 <code>ctrl + a</code> 全选即可。 如果已经发布, 则可以点击修改， 进入修改页然后在复制。</p><hr/><p><br></p><p style="text-align: start;">关于文章的附图, 支持直接上传和复制网络图片</p><p style="text-align: start;"><strong>复制网络图片的方法: </strong>右键你需要的图片, 点击【复制图片】按钮 ， 即可在鄂助教的富文本编辑器中 <code>ctrl+v</code> 进行粘贴。</p><hr/><p><br></p><p style="text-align: start;">更多注意事项可以查看 <a href="https://www.baidu.com" target="">发文守则</a></p>`,
  });
  const methods = reactive({
    getArticleInfo: async () => {
      const res = await articles.queryArticleInfo();
      if (res.code == 200) {
        res.data.type = '文章';
        state.data[0] = res.data;
      }
    },
    init: async function () {
      Promise.allSettled([methods.getArticleInfo()]);
    },
  });
  methods.init();
</script>
