<template>
  <div class="login-box">
    <div class="login-welcome">
      <a-image width="60" :src="logo" />
      <h2 class="login-title">欢迎来到鄂助教</h2>
    </div>
    <div class="login-panel">
      <div class="panel-content">
        <div class="panel-input">
          <a-input v-model="identity" placeholder="账号/电话/邮箱" size="large" allow-clear :max-length="100">
            <template #prefix>
              <icon-idcard />
            </template>
          </a-input>
          <a-input-password v-model="password" placeholder="请输入密码" size="large" allow-clear :max-length="100">
            <template #prefix>
              <IconLock />
            </template>
          </a-input-password>
          <f-captcha v-model="captchaValue" :captchaImg="captcha.img" @click-img="getCaptcha" placeholder="请输入验证码"></f-captcha>
          <div class="forget-pwd-tip">
            <span class="forget-pws-btn" @click="onForget">忘记密码？</span>
          </div>
        </div>
        <a-button type="primary" long :loading="loading" @click="onLogin">登录</a-button>
      </div>
    </div>
    <div class="register-tip">还没有账号？<span class="register-btn" @click="onRegister">立即注册</span></div>
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue';
  import logo from '@/assets/logo.png';
  import useRouter from '@/hooks/useRouter';
  import FCaptcha from '@/components/Captcha.vue';
  import login from '@/api/modules/login';
  import type { Captcha, LoginDTO } from '@/api/types/login';
  import { useUserStore } from '@/stores/modules/user';
  import { Message } from '@arco-design/web-vue';

  const router = useRouter();
  // 验证码
  const captcha = ref({
    uuid: '',
    img: 'http://file.urainstar.top/error.png',
  } as Captcha);
  const loading = ref(false);
  const user = useUserStore();
  const identity = ref('admin');
  const password = ref('123456');
  const captchaValue = ref('');
  // 获取验证码
  const getCaptcha = () => {
    login.captcha().then(res => {
      captcha.value = res.data;
    });
  };
  getCaptcha();
  // 忘记密码
  const onForget = () => {
    router.push('/reset');
  };
  // 注册按钮
  const onRegister = () => {
    router.push('/register');
  };
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
        if (res.code == 200) {
          user.setUser(res.data).then(_res => {
            router.push('/home');
            Message.info(`登录成功，欢迎 ${identity.value}`);
          });
        } else {
          getCaptcha();
        }
      })
      .finally(() => {
        loading.value = false;
      });
  };
</script>

<style scoped lang="scss">
  .login-box {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    margin-bottom: 5%;
    .login-welcome {
      display: flex;
      align-items: center;
      justify-content: center;
      margin-bottom: 40px;
    }
    .login-title {
      margin-left: 10px;
      text-align: center;
    }
    .login-panel {
      align-items: center;
      padding: 30px 0;
      background-color: #fff;
      width: 450px;
      border-radius: 5px;
      box-shadow: 0 1px 2px rgb(171, 171, 171);
      border-top: 2px solid #003cff;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      .panel-content {
        width: 80%;
        .panel-input {
          margin-bottom: 20px;
          .forget-pwd-tip {
            margin-top: 10px;
            display: flex;
            justify-content: flex-end;
            font-size: 15px;
            .forget-pws-btn {
              color: #003cff;
              cursor: pointer;
            }
          }
        }
      }
      .arco-input-wrapper {
        margin-bottom: 20px;
      }
    }
    .register-tip {
      margin-top: 50px;
      display: flex;
      justify-content: center;
      font-size: 15px;
      color: #7f7f7f;
      .register-btn {
        color: #003cff;
        cursor: pointer;
      }
    }
  }
</style>
