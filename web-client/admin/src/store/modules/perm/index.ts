import { Names } from '@/store/types/store-name';
import { defineStore } from 'pinia';
import type { RouteRecordRaw } from 'vue-router';
import useUserStore from '../user';
import { generateRoutes } from './utils';
import router, { defaultRoutes } from '@/router';

const usePermStore = defineStore(Names.PERM, {
  state: () => {
    return {
      permRoutes: [] as Array<RouteRecordRaw>,
    };
  },

  getters: {
    getPermSideBar(state) {
      return state.permRoutes.filter(route => route.meta);
    },
    getPermSplitTabs(state) {
      return state.permRoutes.filter(route => route.meta && route.children && route.children.length > 0);
    },
    getTopLevelTabs(state) {
      return state.permRoutes
        .filter(route => {
          return route.meta && route.children && route.children.length > 0;
        })
        .map(route => {
          const obj = { ...route, items: route.children };
          delete obj.children;
          return obj;
        });
    },
  },

  actions: {
    async setPermRoutes() {
      const accessRoutes = await generateRoutes(useUserStore().user);
      accessRoutes.forEach(route => {
        router.addRoute(route);
      });
      this.permRoutes = [...defaultRoutes, ...accessRoutes];
    },
    emptyPermRoutesFlag() {
      return !this.permRoutes || this.permRoutes.length == 0;
    },
  },
  persist: {
    enable: true,
    restoreState: true,
  },
});
export default usePermStore;
