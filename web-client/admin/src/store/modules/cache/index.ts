import { Names } from '@/store/types/store-name';
import { toHump } from '@/utils';
import { defineStore } from 'pinia';

const useCachedRouteStore = defineStore(Names.CACHE, {
  state: () => {
    return {
      cachedRoutes: [] as string[],
    };
  },
  getters: {
    getCachedRouteName(state) {
      return state.cachedRoutes;
    },
  },
  actions: {
    initCachedRoute(routes: string[]) {
      this.cachedRoutes = routes.map(it => {
        return toHump(it as string);
      });
    },
    setCachedRoutes(cachedRoutes: string[] = []) {
      this.cachedRoutes = cachedRoutes;
    },
    resetCachedRoutes() {
      this.$reset();
    },
  },
});

export default useCachedRouteStore;
