<template>
  <div class="header">
    <div class="left-panel">
      <div class="logo">
        <a-image width="60" :src="logo" />
      </div>
      <div class="sections">
        <span class="section" v-for="(item, index) in sections" :key="index">{{ item }}</span>
      </div>
    </div>
    <div class="right-panel">
      <div class="search">
        <a-input-search
          :style="{ width: '200px' }"
          allow-clear
          placeholder="搜索..."
          @press-enter="onSearch"
          @search="onSearch"
          :loading="searchLoading"
        />
      </div>
      <a-divider direction="vertical" />
      <div class="user">
        <a-popover title="昵称" content-class="user-popover">
          <a-avatar class="avatar" :size="35">
            <img style="border-radius: 50%" alt="头像" src="http://file.urainstar.top/avatar1.jpg" />
          </a-avatar>
          <template #content>
            <div class="profile">个人信息</div>
            <div class="exit">退出登录</div>
          </template>
        </a-popover>
      </div>
      <a-divider direction="vertical" />
      <div class="message">
        <icon-notification size="22" />
      </div>
    </div>
  </div>
  <div class="footer">
    <div class="divider"></div>
    <div class="footer-box">
      <div class="footer-left">
        <div class="footer-title">
          <h3>鄂助教</h3>
          <span>享受文明健康的校园生活 ——</span>
        </div>
        <div class="footer-link">
          <a>关于我们</a>
          <a>加入我们</a>
          <a>联系方式</a>
          <a>隐私政策</a>
        </div>
        <div class="footer-content">本站点内容均来自于公开网络或用户自行创建，在未获得创作者本人同意时禁止以任意形式转载、分发。</div>
      </div>
      <div class="footer-right">
        <div class="footer-contact">
          <div class="qq-icon">
            <icon-qq size="30" />
          </div>
          <div class="wechat-icon">
            <icon-wechat size="35" />
          </div>
        </div>
      </div>
    </div>
    <div class="footer-statement">©E-Tech-Assist. All Rights Reserved! · version: v0.0.0</div>
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue';
  import logo from '../../assets/logo.png';
  const sections = ref(['首页', '文章', '资讯', '聊天室']);
  const searchLoading = ref(false);
  const onSearch = () => {
    searchLoading.value = true;
  };
</script>

<style scoped lang="scss">
  .header {
    width: 100%;
    height: 70px;
    position: fixed;
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
            transition: all 0.3s ease-in-out;
          }
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
      .user {
        margin: 0 25px;
        .avatar {
          cursor: pointer;
        }
      }
      .message {
        cursor: pointer;
        margin-left: 25px;
        margin-top: 5px;
        &:hover {
          color: #006fff;
          transition: all 0.3s ease-in-out;
        }
      }
    }
  }

  .footer {
    position: fixed;
    bottom: 0;
    width: 100%;
    .divider {
      height: 5px;
      background-color: #006fff;
    }
    .footer-box {
      margin-top: 20px;
      padding: 0 10%;
      display: flex;
      align-items: center;
      justify-content: space-between;
      .footer-left {
        .footer-title {
          display: flex;
          align-items: center;
          span {
            margin-left: 10px;
            font-size: 10px;
          }
        }
        .footer-link {
          margin-top: 10px;
          a {
            margin-right: 10px;
            cursor: pointer;
            font-size: 10px;
            color: #a4a4a4;
            &:hover {
              color: #006fff;
              transition: all 0.3s ease-in-out;
            }
          }
        }
        .footer-content {
          margin-top: 10px;
          font-size: 10px;
          color: #a4a4a4;
        }
      }
      .footer-right {
        .footer-contact {
          display: flex;
          align-items: center;
          float: right;
          margin-right: 10%;
          .qq-icon,
          .wechat-icon {
            margin-left: 25px;
            cursor: pointer;
            &:hover {
              color: #006fff;
              transition: all 0.3s ease-in-out;
            }
          }
        }
      }
    }
    .footer-statement {
      margin-top: 20px;
      margin-bottom: 10px;
      font-size: 12px;
      text-align: center;
    }
  }
</style>
