<template>
  <div class="reset-box">
    <div class="reset-welcome">
      <a-image width="60" :src="logo" />
    </div>
    <div class="reset-panel">
      <h2 class="panel-title">重置密码</h2>
      <div class="panel-content">
        <div class="panel-input">
          <a-input v-model="identity" placeholder="账号/电话/邮箱" size="large" allow-clear :max-length="100">
            <template #prefix>
              <icon-idcard />
            </template>
          </a-input>
          <a-input-password v-model="newPassword" placeholder="请输入密码" size="large" allow-clear :max-length="100">
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
        <a-button type="primary" long :loading="loading" @click="onResetPwd">重置</a-button>
      </div>
    </div>
    <div class="login-tip"><span @click="onLogin">返回登录</span></div>
  </div>
</template>

<script setup lang="ts">
  import login from '@/api/modules/login';
  import type { Captcha, ResetPwdDTO } from '@/api/types/login';
  import logo from '@/assets/logo.png';
  import useRouter from '@/hooks/useRouter';
  import { Message } from '@arco-design/web-vue';
  import { ref } from 'vue';
  import FCaptcha from '@/components/Captcha.vue';
  const loading = ref(false);
  const router = useRouter();
  const identity = ref('');
  const newPassword = ref('');
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
  // 找回密码
  const onResetPwd = () => {
    loading.value = true;
    login
      .resetPwd({
        identity: identity.value,
        newPassword: newPassword.value,
        phoneNumber: phoneNumber.value,
        captcha: captchaValue.value,
        uuid: captcha.value.uuid,
      } as ResetPwdDTO)
      .then(res => {
        if (res.code == 200) {
          router.back();
          Message.info(`重置成功`);
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
  .reset-box {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    margin-bottom: 5%;
    .reset-welcome {
      display: flex;
      align-items: center;
      justify-content: center;
      margin-bottom: 40px;
    }
    .reset-panel {
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
