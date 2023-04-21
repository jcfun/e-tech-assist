import type { DeviceType } from '@/setting/types';
import type { Ref, UnwrapRef } from 'vue';
import type { RouteRecordRaw } from 'vue-router';

export enum LayoutMode {
  LTR = 'ltr',
  LCR = 'lcr',
  TTB = 'ttb',
}

export enum ThemeMode {
  LIGHT = 'light',
  DARK = 'dark',
}

export enum SideTheme {
  DARK = 'dark',
  WHITE = 'white',
  BLUE = 'blue',
  IMAGE = 'image',
}

export enum PageAnim {
  FADE = 'fade',
  OPACITY = 'opacity',
  DOWN = 'down',
  SCALE = 'scale',
}

export interface AppConfigState {
  projectName: string;
  theme: ThemeMode;
  sideTheme: SideTheme;
  themeColor: string;
  layoutMode: LayoutMode;
  deviceType: DeviceType;
  sideWidth: number;
  pageAnim: PageAnim;
  isFixedNavBar: boolean;
  isCollapse: boolean;
  actionBar: {
    isShowSearch: boolean;
    isShowMessage: boolean;
    isShowRefresh: boolean;
    isShowFullScreen: boolean;
  };
}

export interface MenuOption {
  key: string | undefined;
  label: string | undefined;
  icon: string | undefined;
  children: Array<MenuOption> | null;
}

export interface SplitTab {
  label: string;
  iconPrefix?: string | unknown;
  icon: string;
  fullPath: string;
  children?: Array<RouteRecordRaw>;
  checked: Ref<UnwrapRef<boolean>>;
}
