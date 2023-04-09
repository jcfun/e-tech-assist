import { createSSRApp } from 'vue';
import uviewPlus from 'uview-plus';
import store from './store/common';
import App from './App.vue';

export function createApp() {
  const app = createSSRApp(App);
  app.use(uviewPlus);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  (uni as any).$u.config.unit = 'rpx';
  app.use(store);
  return {
    app,
  };
}
