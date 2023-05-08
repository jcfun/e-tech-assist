import useUserStore from '@/store/modules/user';
import router from '..';
import usePermStore from '@/store/modules/perm';
import { Message } from '@arco-design/web-vue';

const whiteRoutes: Array<string> = ['/login', '/404', '403', '/500'];

const usePermGuard = () => {
  router.beforeEach(async to => {
    if (whiteRoutes.includes(to.path)) {
      return true;
    }
    const userStore = useUserStore();
    if (userStore.tokenExpireFlag()) {
      Message.error({
        content: '登录已过期',
        duration: 2000,
      });
      return { path: '/login', query: { redirect: '/' }, replace: true };
    }
    const permStore = usePermStore();
    const emptyPermRoutesFlag = permStore.emptyPermRoutesFlag();
    if (emptyPermRoutesFlag) {
      await permStore.setPermRoutes();
      return { ...to, replace: true };
    }
    return true;
  });
};

export default usePermGuard;
