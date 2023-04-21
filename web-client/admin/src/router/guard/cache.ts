import { findCachedRoutes } from '@/store/modules/perm/utils';
import useCachedRouteStore from '@/store/modules/cache';
import router from '..';

function useCachedGuard() {
  router.beforeEach(() => {
    const cachedRouteStore = useCachedRouteStore();
    if (cachedRouteStore.getCachedRouteName.length === 0) {
      cachedRouteStore.initCachedRoute(findCachedRoutes(router.getRoutes()));
    }
    return true;
  });
}

export default useCachedGuard;
