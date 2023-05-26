import { createRouter, createWebHistory } from 'vue-router';
import type { App } from 'vue';
import { initGuard } from './guard';
const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'Layout',
      redirect: '/home',
      component: () => import('@/components/Layout.vue'),
      children: [
        {
          path: '/login',
          name: 'Login',
          component: () => import('@/views/login/index.vue'),
          meta: { transitionEnter: 'animate__zoomIn', transitionLeave: 'animate__fadeOut', authFlag: false },
        },
        {
          path: '/register',
          name: 'Register',
          component: () => import('@/views/register/index.vue'),
          meta: { transitionEnter: 'animate__zoomIn', transitionLeave: 'animate__fadeOut', authFlag: false },
        },
        {
          path: '/reset',
          name: 'Reset',
          component: () => import('@/views/reset-pwd/index.vue'),
          meta: { transitionEnter: 'animate__zoomIn', transitionLeave: 'animate__fadeOut', authFlag: false },
        },
        {
          path: '/home',
          name: 'Home',
          component: () => import('@/views/home/index.vue'),
          meta: { authFlag: false },
        },
        {
          path: '/article',
          name: 'Article',
          component: () => import('@/views/article/index.vue'),
          meta: { authFlag: false },
        },
        {
          path: '/article/detail',
          name: 'ArticleDetail',
          component: () => import('@/views/article/detail.vue'),
          meta: { authFlag: false },
        },
        {
          path: 'article/search',
          name: 'ArticleSearch',
          component: () => import('@/views/article/search.vue'),
          meta: { authFlag: false },
        },
        {
          path: '/create-center/article/edit',
          name: 'ArticleEdit',
          component: () => import('@/views/create-center/article/edit.vue'),
          meta: { authFlag: true },
        },
        {
          path: '/create-center',
          name: 'CreateCenter',
          component: () => import('@/views/create-center/index.vue'),
          meta: { authFlag: true },
          children: [
            {
              path: '/create-center/overview',
              name: 'Overview',
              component: () => import('@/views/create-center/overview/index.vue'),
              meta: { authFlag: true },
            },
            {
              path: '/create-center/article/overview',
              name: 'ArticleOverview',
              component: () => import('@/views/create-center/article/overview.vue'),
              meta: { authFlag: true },
            },
            {
              path: '/create-center/article/publish',
              name: 'ArticlePublish',
              component: () => import('@/views/create-center/article/publish.vue'),
              meta: { authFlag: true },
            },
          ],
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
