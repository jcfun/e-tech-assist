import { npAction } from '@/styles';
import { Message } from '@arco-design/web-vue';
import type { Router } from 'vue-router';
import { useUserStore } from '@/stores/modules/user';

// 在过渡动画结束时移除disable-scroll
document.documentElement.addEventListener('transitionend', () => {
  document.documentElement.classList.remove('disable-scroll');
});

export const initGuard = (router: Router) => {
  router.beforeEach((to, _from) => {
    if (to.meta.authFlag && !useUserStore()?.user?.token) {
      // 如果需要登录权限
      Message.info('请先登录');
      return false;
    }
    // 在路由切换时，禁用滚动条，防止因过渡动画导致的滚动条跳动
    document.documentElement.classList.add('disable-scroll');
    npAction.start();
    return true;
  });

  router.afterEach((_to, _from) => {
    npAction.done();
    return true;
  });
};
