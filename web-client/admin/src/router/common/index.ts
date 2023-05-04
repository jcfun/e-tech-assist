import type { RouteRecordRaw } from 'vue-router';

export const commonRoutes: RouteRecordRaw[] = [
  {
    path: '/login',
    name: 'Login',
    component: () => import('@/views/login/index.vue'),
    meta: {
      title: '登录',
      hidden: true,
    },
  },
  {
    path: '/redirect',
    name: 'Layout',
    component: () => import('@/layout/Layout.vue'),
    meta: {
      hidden: true,
      noShowTabbar: true,
    },
    children: [
      {
        path: '/redirect/:path(.*)*',
        component: () => import('@/views/redirect/index.vue'),
      },
    ],
  },
  {
    path: '/personal',
    name: 'Personal',
    component: () => import('@/layout/Layout.vue'),
    meta: {
      title: '个人中心',
      hidden: true,
    },
    children: [
      {
        path: 'info',
        component: () => import('@/views/index/personal/index.vue'),
        meta: {
          title: '个人中心',
        },
      },
    ],
  },
];

/**
 * 这个路由是为了防止vue-router在一开始匹配不到路由的时候报：
 * No match found for location with xxx 的警告
 */
export const perLoadPathRoute = {
  path: window.location.hash.includes('redirect=/') ? '/' : window.location.hash.split('?')[0].replace('#', '') || window.location.pathname,
  name: 'perLoadPathRouteName',
  component: () => import('@/views/redirect/preload-route.vue'),
  meta: {
    hidden: true,
    noShowTabbar: true,
  },
};
