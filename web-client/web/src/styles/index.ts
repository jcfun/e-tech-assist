import ArcoVue from '@arco-design/web-vue';
import '@arco-design/web-vue/dist/arco.css';
import ArcoVueIcon from '@arco-design/web-vue/es/icon';
import type { App } from 'vue';
import '@/assets/base.scss';

export const initStyle = (app: App) => {
  app.use(ArcoVue);
  app.use(ArcoVueIcon);
};
