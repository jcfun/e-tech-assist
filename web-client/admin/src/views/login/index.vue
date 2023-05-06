<template>
  <div class="login-container">
    <img src="../../assets/bg.png" class="bg-img" />
    <div class="center">
      <div class="left">
        <img src="../../assets/bg_left.png" class="left-bg-img" />
      </div>
      <div class="form-wrapper">
        <div class="title">账号登录</div>
        <div class="item-wrapper mt-10">
          <a-input v-model="identity" placeholder="请输入账号/手机号/邮箱" allow-clear size="large">
            <template #prefix>
              <IconMobile />
            </template>
          </a-input>
        </div>
        <div class="item-wrapper mt-6">
          <a-input-password v-model="password" placeholder="请输入密码" allow-clear size="large">
            <template #prefix>
              <IconLock />
            </template>
          </a-input-password>
        </div>
        <div class="item-wrapper mt-6 captcha">
          <a-input v-model="captchaValue" placeholder="请输入验证码" allow-clear size="large">
            <template #prefix>
              <IconExclamationCircle />
            </template>
          </a-input>
          <div class="captcha-img" @click="getCaptcha">
            <a-image :preview="false" :src="captcha.img" height="35" width="90" alt="验证码加载失败" />
          </div>
        </div>
        <div class="mt-20">
          <a-button type="primary" class="login" :loading="loading" @click="onLogin"> 登录 </a-button>
        </div>
        <div class="my-width mt-6 mb-20">
          <div class="flex justify-between">
            <a-checkbox v-model="autoLogin">自动登录</a-checkbox>
            <a-link :underline="false" type="primary">忘记密码？</a-link>
          </div>
        </div>
        <a-divider orientation="center">第三方登录</a-divider>
        <div class="text-center text-lg">
          <IconAlipayCircle />
          <IconGithub class="mr-6 ml-6" />
          <IconWeiboCircleFill />
        </div>
      </div>
    </div>
    <div class="bottom">{{ 'Vue3 + Vite + Typescript + Arco Design  ©   ' + projectName + '    ' + version + ' · Made by jcfun' }}</div>
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue';
  import setting from '@/setting';
  import login from '@/api/requests/login';
  import type { Captcha, LoginDTO } from '@/api/types/login';
  import { Message } from '@arco-design/web-vue';
  import useUserStore from '@/store/modules/user';
  import router from '@/router';
  import useAppInfo from '@/hooks/useAppInfo';
  import { useRoute } from 'vue-router';
  const autoLogin = ref(true);
  const loading = ref(false);
  const projectName = setting.projectName;
  const { version } = useAppInfo();
  const user = useUserStore();
  const route = useRoute();
  const identity = ref('admin');
  const password = ref('123456');
  const captchaValue = ref('');
  const captcha = ref({
    uuid: 'http://file.urainstar.top/error.png',
    img: '',
  } as Captcha);

  // 获取验证码
  const getCaptcha = () => {
    login.captcha().then(res => {
      captcha.value = res.data;
    });
  };
  getCaptcha();

  // 登录
  const onLogin = () => {
    loading.value = true;
    login
      .login({
        identity: identity.value,
        password: password.value,
        captcha: captchaValue.value,
        uuid: captcha.value.uuid,
      } as LoginDTO)
      .then(res => {
        user.setUser(res.data).then(_res => {
          router.replace({
            path: route.query.redirect ? (route.query.redirect as string) : '/',
          });
          Message.success(`登录成功，欢迎 ${identity.value}，即将跳转到首页`);
        });
      })
      .catch(err => {
        console.log(err);
        getCaptcha();
        Message.error(err);
      });
    loading.value = false;
  };
</script>

<style lang="scss" scoped>
  $leftWith: 35%;
  .login-container {
    position: relative;
    overflow: hidden;
    height: 100vh;
    width: 100vw;
    display: flex;
    justify-content: center;
    align-items: center;
    .bg-img {
      width: 100%;
      height: 100%;
      position: absolute;
      top: 0;
      left: 0;
    }
    .bottom {
      position: fixed;
      left: 0;
      right: 0;
      bottom: 3%;
      font-size: 14px;
      font-weight: bold;
      color: #333;
      text-align: center;
    }
    .center {
      position: relative;
      z-index: 9;
      width: 70%;
      height: 70%;
      border-radius: 10px;
      border: 1px solid #f5f5f5;
      display: flex;
      align-items: center;
      background-color: #fff;
      box-shadow: 0 0 5px #ececec;
      .left {
        position: relative;
        width: 50%;
        height: 100%;
        padding: 20px;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        text-align: center;
        .left-bg-img {
          width: 100%;
          height: 100%;
          position: absolute;
          top: 0;
          left: 0;
        }
        .proj-name {
          font-size: 30px;
          font-weight: bold;
          flex: 1;
          display: flex;
          align-items: center;
          justify-content: center;
        }
      }
      .form-wrapper {
        width: 50%;
        padding: 2% 5%;
        height: 100%;
        display: flex;
        flex-direction: column;
        justify-content: center;
        .title {
          font-size: 25px;
          font-weight: bold;
          margin-bottom: 20px;
          text-align: center;
        }
        .login {
          width: 100%;
        }

        .captcha {
          display: flex;
          flex-direction: row;
          .arco-input-wrapper {
            margin-right: 5px;
          }
          .captcha-img {
            display: flex;
            justify-content: center;
            .arco-image {
              :deep(.arco-image-img) {
                max-height: 35px;
                max-width: none;
              }
              :deep(.arco-image-overlay) {
                .arco-image-error {
                  justify-content: start;
                }
              }
            }
          }
        }
      }
    }
  }
  @media (max-width: 966px) {
    .left {
      display: none;
    }
    .right {
      background-image: url('../../assets/img_login_mobile_bg_01.jpg');
      background-size: cover;
      .form-wrapper {
        width: 80% !important;
      }
    }
  }
</style>
