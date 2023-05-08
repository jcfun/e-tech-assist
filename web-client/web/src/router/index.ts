import { createRouter, createWebHistory } from 'vue-router';
import type { App } from 'vue';
const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'Layout',
      component: () => import('@/components/layout/Layout.vue'),
    },
  ],
});

export const initRouter = (app: App) => {
  app.use(router);
};
