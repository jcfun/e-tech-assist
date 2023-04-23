import { Names } from '@/store/types/store-name';
import { defineStore } from 'pinia';
import type { RouteRecordRaw } from 'vue-router';
import useUserStore from '../user';
import { findRootPathRoute, generateRoutes, mapTwoLevelRouter } from './utils';
import router from '@/router';
import asyncRoutes from '@/router/async';
import { commonRoutes } from '@/router/common';

const usePermStore = defineStore(Names.PERM, {
  state: () => {
    return {
      permRoutes: [] as Array<RouteRecordRaw>,
    };
  },

  getters: {
    getPermSideBar(state) {
      return state.permRoutes.filter(route => route.meta && !route.meta.hidden);
    },
    getPermSplitTabs(state) {
      return state.permRoutes.filter(route => route.meta && route.children && route.children.length > 0 && !route.meta.hidden);
    },
    getTopLevelTabs(state) {
      return state.permRoutes
        .filter(route => {
          return route.meta && route.children && route.children.length > 0 && !route.meta.hidden;
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
      const accessRoutes = mapTwoLevelRouter([...asyncRoutes, ...generateRoutes(useUserStore().user)]);
      accessRoutes.forEach(route => {
        router.addRoute(route);
      });
      // 配置 `/` 路由的默认跳转地址
      router.addRoute({
        path: '/',
        redirect: findRootPathRoute(accessRoutes),
        meta: {
          hidden: true,
        },
      });
      this.permRoutes = [...commonRoutes, ...accessRoutes];
    },
    emptyPermRoutesFlag() {
      return !this.permRoutes || this.permRoutes.length == 0;
    },
  },
  // persist: {
  //   enable: true,
  //   restoreState: true,
  // },
});
export default usePermStore;
