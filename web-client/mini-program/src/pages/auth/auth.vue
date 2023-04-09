<template>
  <view class="auth-box">
    <view class="welcome">欢迎使用 鄂助教</view>
    <view class="info">授权微信头像、昵称</view>
    <view class="icon">
      <u-image :showLoading="true" :src="src" mode="widthFix" width="100%" height="100%"></u-image>
    </view>
    <view class="info">为提供优质服务, 鄂助教需要获取你的以下信息:</view>
    <view class="list-item">
      <view class="dot"></view>
      <view class="list-text">你的公开信息(头像、昵称等)</view>
    </view>
    <view class="auth-btn">
      <u-button type="primary" text="授权进入鄂助教" shape="circle" @tap="toAuth"></u-button>
    </view>
  </view>
  <u-notify ref="authMsg"></u-notify>
  <u-modal :show="needPhone" showCancelButton title="您未使用过微信授权，请输入手机号" @confirm="confirmPhone" @cancel="cancelPhone">
    <u-input placeholder="请输入内容" v-model="phoneNumber" type="number" clearable maxlength="11"></u-input>
  </u-modal>
</template>

<script setup lang="ts">
  import type { _NotifyRef } from '@ttou/uview-typings/types/notify';
  import { ref } from 'vue';
  import { login, register } from '@/api/login';
  import type { LoginDTO, RegisterDTO } from '@/models/login';
  import { useUserStore } from '@/store/user';

  const src = '/static/images/icon/icon.png';
  const authMsg = ref<_NotifyRef>();

  let needPhone = ref(false);

  let phoneNumber = ref('');

  const user = useUserStore();

  // 确认输入手机号后的处理事件
  let registerDTO = ref(<RegisterDTO>{});
  const confirmPhone = () => {
    needPhone.value = false;
    registerDTO.value.phoneNumber = phoneNumber.value;
    console.log('registerDTOvalue====>', registerDTO.value);
    register(registerDTO.value)
      .then(res => {
        if (res.code != '200') {
          authMsg.value?.show({
            type: 'error',
            message: res.msg,
            duration: 2000,
            fontSize: 25,
          });
          return;
        } else {
          user.setToken(res.data.token);
          user.setUserInfo(res.data.userInfo);
        }
        authMsg.value?.show({
          type: 'success',
          message: '授权成功',
          duration: 2000,
          fontSize: 25,
        });
        setTimeout(() => {
          uni.switchTab({
            url: '/pages/mine/mine',
          });
        }, 1000);
      })
      .catch(err => {
        needPhone.value = false;
        console.log(err);
      });
  };
  // 取消输入手机号后的处理事件
  const cancelPhone = () => {
    needPhone.value = false;
    authMsg.value?.show({
      type: 'error',
      message: '取消授权',
      duration: 2000,
      fontSize: 25,
    });
    setTimeout(() => {
      uni.switchTab({
        url: '/pages/mine/mine',
      });
    }, 1000);
  };

  const toAuth = () => {
    let code = ref('');
    if (user.token.token) {
      uni.switchTab({
        url: '/pages/mine/mine',
      });
      return;
    }
    //登录
    uni.login({
      provider: 'weixin',
      success: loginRes => {
        console.log('login成功', loginRes);
        code.value = loginRes.code;
      },
      fail: err => {
        console.log(err);
      },
    });
    uni.getUserProfile({
      desc: '获取你的昵称、头像、地区及性别',
      success: getUserProfileRes => {
        console.log(getUserProfileRes);
        const loginDTO: LoginDTO = {
          code: code.value,
        };
        login(loginDTO)
          .then(res => {
            console.log('loginRes====>', res);
            if (res.code != '200') {
              registerDTO.value = {
                sessionKey: res.data.userInfo.sessionKey,
                encryptedData: getUserProfileRes.encryptedData,
                iv: getUserProfileRes.iv,
                openid: res.data.userInfo.openid,
                phoneNumber: phoneNumber.value,
              };
              needPhone.value = true;
            } else {
              user.setToken(res.data.token);
              user.setUserInfo(res.data.userInfo);
              authMsg.value?.show({
                type: 'success',
                message: '授权成功',
                duration: 2000,
                fontSize: 25,
              });
              setTimeout(() => {
                uni.switchTab({
                  url: '/pages/mine/mine',
                });
              }, 1000);
            }
          })
          .catch(err => {
            console.log(err);
          });
      },
      fail: _fail => {
        console.log('fail');
        authMsg.value?.show({
          type: 'error',
          message: '取消授权',
          duration: 2000,
          fontSize: 25,
        });
        setTimeout(() => {
          uni.switchTab({
            url: '/pages/mine/mine',
          });
        }, 1000);
      },
    });
  };
</script>

<style scoped lang="scss">
  .auth-box {
    margin-left: 25rpx;

    .welcome {
      font-size: 50rpx;
      color: #000000;
      height: 50rpx;
      margin-top: 5%;
    }

    .info {
      font-size: 25rpx;
      color: #000000;
      height: 25rpx;
      margin-top: 5%;
    }

    .icon {
      margin-top: 10%;
      margin-left: -25rpx;
    }

    .list-item {
      display: flex;
      align-items: center;
      margin-top: 20rpx;
      .dot {
        width: 5rpx;
        height: 5rpx;
        border-radius: 50%;
        background-color: black;
        margin-right: 10rpx;
      }

      .list-text {
        font-size: 20rpx;
      }
    }

    .auth-btn {
      margin-right: 25rpx;
      margin-top: 50rpx;
    }
  }
</style>
