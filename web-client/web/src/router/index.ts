import { createRouter, createWebHistory } from 'vue-router';
import type { App } from 'vue';
import { initGuard } from './guard';
const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'Layout',
      component: () => import('@/components/Layout.vue'),
      children: [
        {
          path: '/login',
          name: 'Login',
          component: () => import('@/views/login/index.vue'),
          meta: { transitionEnter: 'animate__zoomIn', transitionLeave: 'animate__fadeOut' },
        },
        {
          path: '/register',
          name: 'Register',
          component: () => import('@/views/register/index.vue'),
          meta: { transitionEnter: 'animate__zoomIn', transitionLeave: 'animate__fadeOut' },
        },
        {
          path: '/reset',
          name: 'Reset',
          component: () => import('@/views/reset-pwd/index.vue'),
          meta: { transitionEnter: 'animate__zoomIn', transitionLeave: 'animate__fadeOut' },
        },
      ],
    },
  ],
});

export default router;

export const initRouter = (app: App) => {
  initGuard(router);
  app.use(router);
};
