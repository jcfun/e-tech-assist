import ArcoVue from '@arco-design/web-vue';
import '@arco-design/web-vue/dist/arco.css';
import ArcoVueIcon from '@arco-design/web-vue/es/icon';
import type { App } from 'vue';
import '@/assets/base.min.css';
import './index.css';
import NProgress from 'nprogress';
import 'nprogress/nprogress.css';
import 'animate.css';

NProgress.configure({
  easing: 'ease', // 动画方式
  speed: 500, // 递增进度条的速度
  showSpinner: false, // 是否显示加载ico
  trickleSpeed: 1000, // 自动递增间隔
  minimum: 0.3, // 初始化时的最小百分比
});

class NpAction {
  public start = () => {
    NProgress.start();
  };

  public done = () => {
    NProgress.done();
  };
}

export const npAction = new NpAction();

export const initStyle = (app: App) => {
  app.use(ArcoVue);
  app.use(ArcoVueIcon);
};
