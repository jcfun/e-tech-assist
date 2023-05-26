<template>
  <div class="header">
    <div class="left-panel">
      <div class="logo" @click="$router.push('/home')">
        <a-image width="60" :preview="false" :src="logo" />
      </div>
      <div class="sections">
        <span
          class="section"
          :class="{ 'sidebar-menu-selected': $route.path == item.path }"
          @click="$router.push(item.path)"
          v-for="(item, index) in sections"
          :key="index"
          >{{ item.title }}</span
        >
      </div>
    </div>
    <div class="right-panel">
      <div class="search">
        <a-input-search
          :style="{ width: '250px' }"
          size="large"
          allow-clear
          placeholder="搜索..."
          @press-enter="onSearch"
          @search="onSearch"
          :loading="searchLoading"
        />
      </div>
      <a-divider direction="vertical" />
      <div class="user-or-login">
        <a-popover v-if="userStore?.user?.token" title="昵称" content-class="user-popover">
          <a-avatar class="avatar" :size="35">
            <img style="border-radius: 50%" alt="头像" :src="userStore?.user?.userInfo?.avatarUrl" />
          </a-avatar>
          <template #content>
            <div class="profile">个人信息</div>
            <div class="create-center" @click="$router.push('/create-center/overview')">创作中心</div>
            <div class="logout" @click="onLogout">退出登录</div>
          </template>
        </a-popover>
        <span v-else class="login" @click="onLogin">登录</span>
      </div>
      <a-divider direction="vertical" />
      <div class="message">
        <icon-notification size="22" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue';
  import logo from '@/assets/logo.png';
  import { useUserStore } from '@/stores/modules/user';
  import router from '@/router';
  import { Message, Modal } from '@arco-design/web-vue';
  const sections = ref([
    {
      title: '首页',
      path: '/home',
    },
    {
      title: '文章',
      path: '/article',
    },
    {
      title: '资讯',
      path: '/information',
    },
    {
      title: '聊天室',
      path: '/chat',
    },
  ]);
  const searchLoading = ref(false);
  const userStore = useUserStore();
  const onSearch = () => {
    searchLoading.value = true;
  };
  const onLogin = () => {
    router.push('/login');
  };
  const onLogout = () => {
    Modal.confirm({
      title: '提示',
      content: '是否要退出当前账号？',
      okText: '退出',
      cancelText: '再想想',
      onOk: () => {
        userStore.logout().then(() => {
          // window.location.reload();
          router.push('/home');
          Message.info('退出成功');
        });
      },
    });
  };
</script>

<style scoped lang="scss">
  .header {
    width: 100%;
    height: 70px;
    position: relative;
    top: 0;
    box-shadow: 0px 2px 5px 0px rgb(236, 236, 236);
    background-color: rgb(255, 255, 255);
    display: flex;
    align-items: center;
    flex-direction: row;
    justify-content: space-between;
    padding: 0 2%;
    .left-panel {
      display: flex;
      align-items: center;
      .logo {
        cursor: pointer;
      }
      .sections {
        .section {
          margin-left: 30px;
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
    .right-panel {
      display: flex;
      align-items: center;
      .search {
        margin: 0 25px;
        .search-icon {
          cursor: pointer;
        }
      }
      .user-or-login {
        margin: 0 25px;
        min-width: 35px;
        .avatar {
          cursor: pointer;
        }
        .login {
          cursor: pointer;
          &:hover {
            color: #006fff;
            transition: all 0.3s ease-in;
          }
        }
      }
      .message {
        cursor: pointer;
        margin-left: 25px;
        margin-top: 5px;
        &:hover {
          color: #006fff;
          transition: all 0.3s ease-in;
        }
      }
    }
  }
</style>
