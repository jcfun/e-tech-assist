import type { LoginVO } from '@/api/types/login';
import type { PermVO } from '@/api/types/perm';
import type { MenuOption, SplitTab } from '@/store/types/layout';
import { Message } from '@arco-design/web-vue';
import { isExternal } from '@/utils';
import { ref } from 'vue';
import type { RouteLocationNormalized, RouteRecordRaw } from 'vue-router';
import { resolve } from 'path-browserify';

// 生成路由结构
export const generateRoutes = (loginVO: LoginVO) => {
  const perms = loginVO.userInfo.perms;
  if (!perms) {
    Message.error('获取用户权限失败');
    return [];
  }
  let routesArr = [] as Array<RouteRecordRaw>;
  const routesTreeArr = getRouteTree(perms);
  routesArr = [...routesTreeArr, ...routesArr];
  return routesArr;
};

// 使用递归构建路由树
const getRouteTree = (perms: Array<PermVO>) => {
  const routers = [] as Array<RouteRecordRaw>;
  perms.forEach(perm => {
    const route: RouteRecordRaw = {
      path: `${perm.parentRoute ?? ''}${perm.feRoute}`,
      name: perm.feName,
      component: perm.permType == '0' ? () => import('@/layout/Layout.vue') : getComponent(perm),
      meta: {
        title: perm.name,
        icon: perm.resource,
        hidden: perm.hiddenFlag == '1',
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
  return components[`/src/views${perm.parentRoute ?? ''}${perm.feRoute}.vue`];
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

export function transformMenu(originRoutes: Array<RouteRecordRaw>): Array<MenuOption> {
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
          tempMenu.children = transformMenu(it.children);
        }
      }
      tempMenus.push(tempMenu);
    });
  return tempMenus;
}

export function findAffixedRoutes(routes: Array<RouteLocationNormalized>) {
  const temp = [] as Array<RouteLocationNormalized>;
  routes.forEach(it => {
    if (it.meta && it.meta.affix) {
      if (!it.fullPath) {
        it.fullPath = it.path;
      }
      temp.push(it);
    }
  });
  return temp;
}

export function mapTwoLevelRouter(srcRoutes: Array<RouteRecordRaw>) {
  function addParentRoute(routes: any, parent: any, parentPath: string) {
    routes.forEach((it: RouteRecordRaw) => {
      if (!isExternal(it.path)) {
        it.path = resolve(parentPath, it.path);
      }
      parent.push(it);
      if (it.children && it.children.length > 0) {
        addParentRoute(it.children, parent, it.path);
      }
    });
  }
  if (srcRoutes && srcRoutes.length > 0) {
    const tempRoutes = [] as Array<any>;
    srcRoutes.forEach(it => {
      const route = { ...it };
      const parentRoutes = [] as Array<any>;
      if (route.children && route.children.length > 0) {
        addParentRoute(route.children, parentRoutes, route.path);
      }
      parentRoutes && parentRoutes.length > 0 && (route.children = parentRoutes);
      tempRoutes.push(route);
    });
    return tempRoutes;
  }
  return [];
}

export function findRootPathRoute(routes: RouteRecordRaw[]) {
  if (!routes || routes.length === 0) {
    Message.error('系统加载菜单发生异常，请打开控制台查看具体原因');
    console.error(
      '系统加载菜单发生异常，有可能是在加载菜单的时候返回了空数据或者接口发生异常，如果您采用前端加载菜单的方式请确保/src/router/routes/default-route.ts文件里面有配置路由',
    );
    return '/login';
  }
  for (let index = 0; index < routes.length; index++) {
    const route = routes[index];
    const rootRoute = route.children?.find(it => it.meta && it.meta.isRootPath);
    if (rootRoute) {
      return rootRoute.path;
    }
  }
  return routes && routes.length > 0 && routes[0].children && routes[0].children.length > 0 ? routes[0].children![0].path : '/login';
}
