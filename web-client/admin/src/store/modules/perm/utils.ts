import type { LoginVO } from '@/api/types/login';
import type { PermVO } from '@/api/types/perm';
import type { MenuOption, SplitTab } from '@/store/types/layout';
import { ref } from 'vue';
import type { RouteLocationNormalized, RouteRecordRaw } from 'vue-router';

export const generateRoutes = (loginVO: LoginVO) => {
  const roles = loginVO.userInfo.roles;
  let routesArr = [] as Array<RouteRecordRaw>;
  roles.forEach(role => {
    const routesTreeArr = getRouteTree(role.perms);
    routesArr = [...routesTreeArr, ...routesArr];
  });
  return routesArr;
};

const getRouteTree = (perms: Array<PermVO>) => {
  const routers = [] as Array<RouteRecordRaw>;
  perms.forEach(perm => {
    const route: RouteRecordRaw = {
      path: perm.feRoute,
      name: perm.feName,
      component: perm.permType == '0' ? () => import('@/layout/Layout.vue') : getComponent(perm),
      meta: {
        title: perm.name,
      },
      children: getRouteTree(perm.children).length > 0 ? getRouteTree(perm.children) : undefined,
    };
    routers.push(route);
  });
  return routers;
};

export function loadComponents() {
  return import.meta.glob('/src/views/**/*.vue');
}

export const components = loadComponents();

export function getComponent(perm: PermVO) {
  return components[`/src/views${perm.feRoute}.vue`];
}

export function transformSplitTabMenu(routes: Array<RouteRecordRaw>): Array<SplitTab> {
  const tempTabs = [] as Array<SplitTab>;
  routes.forEach(route => {
    const splitTab: SplitTab = {
      label: route.meta ? (route.meta?.title as string) : '',
      fullPath: route.path || '',
      iconPrefix: route.meta?.iconPrefix || 'icon',
      icon: route.meta ? (route.meta?.icon as any) : undefined,
      children: route.children as RouteRecordRaw[],
      checked: ref(false),
    };
    tempTabs.push(splitTab);
  });
  return tempTabs;
}

export function findCachedRoutes(routes: Array<RouteLocationNormalized>) {
  const temp = [] as Array<string>;
  routes.forEach(it => {
    if (it.name && it.meta && it.meta.cacheable) {
      temp.push(it.name as string);
    }
  });
  return temp;
}

export function transfromMenu(originRoutes: Array<RouteRecordRaw>): Array<MenuOption> {
  if (!originRoutes) {
    return [];
  }
  const tempMenus = [] as Array<MenuOption>;
  originRoutes
    .filter(it => (it.meta ? !it.meta.hidden : true))
    .forEach(it => {
      const tempMenu: MenuOption = {
        key: it.path,
        label: it.meta?.title as string,
        icon: it.meta?.icon as string,
        children: null,
      };
      if (it.children) {
        if (it.meta && it.meta.isSingle && it.children.length === 1) {
          const lastItem = it.children[0] as RouteRecordRaw;
          tempMenu.key = lastItem.path || tempMenu.key;
          tempMenu.label = (lastItem.meta && lastItem.meta.title ? lastItem.meta?.title : tempMenu.label) as string;
          tempMenu.icon = (lastItem.meta && lastItem.meta.icon ? lastItem.meta?.icon : tempMenu.icon) as string;
          tempMenu.children = null;
        } else {
          tempMenu.children = transfromMenu(it.children);
        }
      }
      tempMenus.push(tempMenu);
    });
  return tempMenus;
}
