import { npAction } from '@/styles';
import type { Router } from 'vue-router';

// 在过渡动画结束时移除disable-scroll
document.documentElement.addEventListener('transitionend', () => {
  document.documentElement.classList.remove('disable-scroll');
});

export const initGuard = (router: Router) => {
  router.beforeEach((_to, _from) => {
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
