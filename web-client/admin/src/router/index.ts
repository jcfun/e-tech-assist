import { createRouter, createWebHistory } from 'vue-router';

export const defaultRoutes = [
  {
    path: '/',
    name: 'Login',
    component: () => import('@/views/login/index.vue'),
    meta: {
      title: '登录',
    },
  },
  {
    path: '/index',
    name: 'Index',
    component: () => import('@/layout/Layout.vue'),
    meta: {
      title: '仪表盘',
    },
    children: [
      {
        path: 'home',
        name: 'Home',
        component: () => import('@/views/index/index.vue'),
        meta: {
          title: '首页',
        },
      },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes: [...defaultRoutes],
});
export default router;
