<template>
  <view class="qrcode-box">
    <view class="user-area">
      <span class="avatar">
        <u-icon v-if="!token.token" name="account-fill" color="#FFFFF8" size="90"></u-icon>
        <u-avatar v-else :src="imgSrc" :size="100" mode="aspectFill"></u-avatar>
      </span>
      <view class="user-info">
        <span class="nickname">{{ isEmpty(user?.userInfo?.nickname) ? '请登录' : user.userInfo.nickname }}</span>
        <span class="account">账号: {{ user?.userInfo?.account }}</span>
      </view>
    </view>
    <canvas id="qrcode" canvas-id="qrcode" style="width: 250px; height: 250px"></canvas>
  </view>
  <view class="operate-box">
    <span class="scan-btn" @click="scan">扫描</span>
    <span class="save-btn" @click="save">保存</span>
  </view>
</template>

<script setup lang="ts">
  import { ref } from 'vue';
  import UQRCode from '@uqrcode/js';
  import { useUserStore } from '@/store/user';
  import { onReady } from '@dcloudio/uni-app';
  import { isEmpty } from '@/utils';
  const user = useUserStore();
  let token = user.token;
  // 头像
  const imgSrc = ref(isEmpty(user?.userInfo?.avatarUrl) ? 'http://file.urainstar.top/logo.png' : user.userInfo.avatarUrl);
  // 二维码内容
  const qrcodeValue = ref(user?.userInfo?.id);
  // 临时文件路径
  const tempFilePath = ref('');
  onReady(() => {
    // 获取uQRCode实例
    let qr = new UQRCode();
    // 预加载图片
    UQRCode.prototype.loadImage = function (src: string) {
      // 需要返回Promise对象
      return new Promise((resolve, reject) => {
        uni.getImageInfo({
          src,
          success: res => {
            // resolve返回img
            resolve(res.path);
          },
          fail: err => {
            // reject返回错误信息
            reject(err);
          },
        });
      });
    };
    // 设置二维码内容
    qr.data = qrcodeValue.value;
    // 设置二维码大小，必须与canvas设置的宽高一致
    qr.size = 250;
    qr.foregroundColor = '#000000';
    qr.backgroundColor = '#f8f8f8';
    qr.style = 'default';
    qr.foregroundImageWidth = '60';
    qr.foregroundImageHeight = '60';
    qr.foregroundImageSrc = isEmpty(imgSrc.value) ? '欢迎使用鄂助教小程序' : imgSrc.value;
    qr.foregroundImageBorderRadius = '5';
    // 调用制作二维码方法
    qr.make();
    // 获取canvas上下文
    let canvasContext = uni.createCanvasContext('qrcode', this); // 如果是组件，this必须传入
    // 设置uQRCode实例的canvas上下文
    qr.canvasContext = canvasContext;
    // 调用绘制方法将二维码图案绘制到canvas上
    qr.drawCanvas();
    // 导出临时文件路径
    setTimeout(() => {
      uni.canvasToTempFilePath(
        {
          canvasId: 'qrcode',
          fileType: 'png',
          width: 250,
          height: 250,
          success: res => {
            tempFilePath.value = res.tempFilePath;
          },
          fail: err => {
            console.log(err);
          },
        },
        //this, // 组件内使用必传当前实例
      );
    }, 300);
  });
  // 保存图片
  const save = () => {
    uni.saveImageToPhotosAlbum({
      filePath: tempFilePath.value,
      success: () => {
        uni.showToast({
          title: '保存成功',
          icon: 'success',
          duration: 2000,
        });
      },
    });
  };
  // 扫码
  const scan = () => {
    uni.scanCode({
      success: success => {
        console.log(success);
      },
    });
  };
</script>

<style scoped lang="scss">
  .qrcode-box {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -75%);
    .user-area {
      display: flex;
      flex-direction: row;
      align-items: center;
      margin-bottom: 30rpx;
      .avatar {
        background-color: rgb(194, 226, 255);
        border-radius: 100%;
        width: 100rpx;
        height: 100rpx;
        display: flex;
        justify-content: center;
        align-items: center;
      }
      .user-info {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        align-items: stretch;
        flex-grow: 1;
        margin-left: 30rpx;
        .nickname {
          font-size: 40rpx;
        }
        .account {
          color: #383838;
          font-size: 20rpx;
        }
      }
    }
  }
  .operate-box {
    position: absolute;
    bottom: 15%;
    left: 50%;
    transform: translateX(-50%);
    width: 100%;
    display: flex;
    flex-direction: row;
    justify-content: center;
    .scan-btn {
      margin: 0 75rpx;
      font-size: 25rpx;
      color: #595959;
      &:hover {
        color: #000000;
      }
    }
    .save-btn {
      margin: 0 75rpx;
      font-size: 25rpx;
      color: #595959;
      &:hover {
        color: #000000;
      }
    }
  }
</style>
