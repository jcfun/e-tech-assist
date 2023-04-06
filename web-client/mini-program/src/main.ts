import { createSSRApp } from "vue";
import uviewPlus from 'uview-plus'
import { createPinia } from 'pinia'
import App from "./App.vue";

const store = createPinia()


export function createApp() {
  const app = createSSRApp(App);
  app.use(uviewPlus)
  app.use(store)
  return {
    app,
  };
}
