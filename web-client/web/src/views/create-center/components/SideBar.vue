<template>
  <div class="sidebar-wrap">
    <div
      class="sidebar-title"
      @click="$router.push('/create-center/overview')"
      :class="{ 'sidebar-menu-selected': $route.path == '/create-center/overview' }"
    >
      总览
    </div>
    <div class="sidebar-content" v-for="(item, index) in menus" :key="index">
      <div class="sidebar-content-title">{{ item.title }}</div>
      <div class="sidebar-menus" v-for="(it, i) in item.children" :key="i">
        <span class="sidebar-menu" :class="{ 'sidebar-menu-selected': $route.path == it.path }" @click="$router.push(it.path)">{{ it.title }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  const menus = [
    {
      title: '文章管理',
      children: [
        {
          title: '文章总览',
          path: '/create-center/article/overview',
        },
        {
          title: '发布文章',
          path: '/create-center/article/publish',
        },
      ],
    },
    {
      title: '资讯管理',
      children: [
        {
          title: '资讯总览',
          path: '/create-center/news/overview',
        },
        {
          title: '发布资讯',
          path: '/create-center/news/publish',
        },
      ],
    },
    {
      title: '聊天室管理',
      children: [
        {
          title: '聊天室总览',
          path: '/create-center/chat/overview',
        },
        {
          title: '创建聊天室',
          path: '/create-center/chat/create',
        },
      ],
    },
  ];
</script>

<style scoped lang="scss">
  .sidebar-wrap {
    // z-index: 9999;
    // position: fixed;
    min-width: 230px;
    height: fit-content;
    background-color: #fff;
    border-radius: 5px;
    box-shadow: 0 0 5px #c9c9c9;
    margin-right: 50px;
    margin-top: 30px;
    .sidebar-title {
      border-bottom: 1px solid #dddddd;
      padding: 10px 20px;
      font-size: 18px;
      cursor: pointer;
      &:hover {
        color: #006fff;
        transition: all 0.3s ease-in;
      }
    }
    .sidebar-content {
      margin: 15px 20px;
      .sidebar-content-title {
        font-size: 16px;
      }
      .sidebar-menus {
        margin-top: 5px;
        .sidebar-menu {
          font-size: 14px;
          color: #7e7e7e;
          cursor: pointer;
          &:hover {
            color: #006fff;
            transition: all 0.3s ease-in;
          }
        }

        .sidebar-menu-selected {
          color: #006fff;
        }
      }
    }
  }

  .sidebar-menu-selected {
    color: #006fff;
  }
</style>
