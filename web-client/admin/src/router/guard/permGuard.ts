import useUserStore from '@/store/modules/user';
import router from '..';
import usePermStore from '@/store/modules/perm';

const whiteRoutes: Array<string> = ['/login', '/404', '403', '/500'];

const usePermGuard = () => {
  router.beforeEach(async to => {
    if (whiteRoutes.includes(to.path)) {
      return true;
    }
    const userStore = useUserStore();
    if (userStore.tokenExpireFlag()) {
      return { path: '/login', query: { push: to.fullPath } };
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
