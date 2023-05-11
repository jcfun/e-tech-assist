<template>
  <div class="register-box">
    <div class="register-welcome">
      <a-image width="60" :src="logo" />
    </div>
    <div class="register-panel">
      <h2 class="panel-title">欢迎注册鄂助教</h2>
      <div class="panel-content">
        <div class="panel-input">
          <a-input v-model="account" placeholder="请输入账号" size="large" allow-clear :max-length="100">
            <template #prefix>
              <icon-idcard />
            </template>
          </a-input>
          <a-input-password v-model="password" placeholder="请输入密码" size="large" allow-clear :max-length="100">
            <template #prefix>
              <IconLock />
            </template>
          </a-input-password>
          <a-input v-model="phoneNumber" placeholder="请输入电话" size="large" allow-clear :max-length="100">
            <template #prefix>
              <IconMobile />
            </template>
          </a-input>
          <f-captcha v-model="captchaValue" :captchaImg="captcha.img" @click-img="getCaptcha" placeholder="请输入验证码"></f-captcha>
        </div>
        <a-button type="primary" long :loading="loading" @click="onRegister">注册</a-button>
      </div>
    </div>
    <div class="login-tip" @click="onLogin">返回登录</div>
  </div>
</template>

<script setup lang="ts">
  import logo from '@/assets/logo.png';
  import useRouter from '@/hooks/useRouter';
  import FCaptcha from '@/components/Captcha.vue';
  import type { Captcha, RegisterDTO } from '@/api/types/login';
  import { ref } from 'vue';
  import login from '@/api/modules/login';
  import { Message } from '@arco-design/web-vue';
  const loading = ref(false);
  const router = useRouter();
  const account = ref('');
  const password = ref('');
  const phoneNumber = ref('');
  const captchaValue = ref('');
  // 验证码
  const captcha = ref({
    uuid: '',
    img: 'http://file.urainstar.top/error.png',
  } as Captcha);
  // 获取验证码
  const getCaptcha = () => {
    login.captcha().then(res => {
      captcha.value = res.data;
    });
  };
  getCaptcha();
  // 返回登录按钮
  const onLogin = () => {
    router.back();
  };
  // 注册
  const onRegister = () => {
    loading.value = true;
    login
      .register({
        account: account.value,
        password: password.value,
        phoneNumber: phoneNumber.value,
        captcha: captchaValue.value,
        uuid: captcha.value.uuid,
      } as RegisterDTO)
      .then(res => {
        if (res.code == 200) {
          router.back();
          Message.info(`注册成功，欢迎 ${account.value}`);
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
  .register-box {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    margin-bottom: 5%;
    .register-welcome {
      display: flex;
      align-items: center;
      justify-content: center;
      margin-bottom: 40px;
    }
    .register-panel {
      background-color: #fff;
      width: 450px;
      padding: 30px 0;
      border-radius: 5px;
      box-shadow: 0 1px 2px rgb(171, 171, 171);
      border-top: 2px solid #003cff;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      .panel-title {
        text-align: center;
        margin-bottom: 20px;
      }
      .panel-content {
        width: 80%;
        .panel-input {
          margin-bottom: 20px;
        }
      }
      .arco-input-wrapper {
        margin-bottom: 20px;
      }
    }
    .login-tip {
      margin-top: 50px;
      display: flex;
      justify-content: center;
      font-size: 15px;
      color: #003cff;
      cursor: pointer;
    }
  }
</style>
