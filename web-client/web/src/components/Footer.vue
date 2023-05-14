<template>
  <div class="footer">
    <div class="divider"></div>
    <div class="footer-box">
      <div class="footer-left">
        <div class="footer-title">
          <h3>鄂助教<span>享受文明健康的校园生活 ——</span></h3>
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

<style scoped lang="scss">
  .footer {
    position: relative;
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
          h3 span {
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
              transition: all 0.3s ease-in;
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
          .qq-icon,
          .wechat-icon {
            margin-left: 25px;
            cursor: pointer;
            &:hover {
              color: #006fff;
              transition: all 0.3s ease-in;
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
