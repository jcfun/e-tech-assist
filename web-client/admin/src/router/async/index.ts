const asyncRoutes = [
  {
    path: '/index',
    name: 'Index',
    component: () => import('@/layout/Layout.vue'),
    meta: {
      title: '仪表盘',
      iconPrefix: 'iconfont',
      icon: 'icon-dashboard',
    },
    children: [
      {
        path: 'home',
        name: 'Home',
        component: (): any => import('@/views/index/index.vue'),
        meta: {
          title: '主控台',
          affix: true,
        },
      },
    ],
  },
];
export default asyncRoutes;
