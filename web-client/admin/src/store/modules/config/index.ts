import { defineStore } from 'pinia';

import defaultSetting from '@/setting';
import type { DeviceType } from '@/setting/types';

import { useChangeMenuWidth } from '@/hooks/useMenuWidth';
import usePrimaryColor from '@/hooks/usePrimaryColor';
import useTheme from '@/hooks/useTheme';
import { Names } from '@/store/types/store-name';
import type { LayoutMode, PageAnim, SideTheme, ThemeMode } from '@/store/types/layout';

const useAppConfigStore = defineStore(Names.APP_CONFIG, {
  state: () => {
    return defaultSetting;
  },
  getters: {
    getLayoutMode(state) {
      return state.layoutMode;
    },
  },
  actions: {
    changeTheme(theme: ThemeMode) {
      this.theme = theme;
      useTheme(theme);
    },
    changeLayoutMode(mode: LayoutMode) {
      this.layoutMode = mode;
    },
    changeDevice(deviceType: DeviceType) {
      this.deviceType = deviceType;
    },
    changeSideBarTheme(sideTheme: SideTheme) {
      this.sideTheme = sideTheme;
    },
    changePageAnim(pageAnim: PageAnim) {
      this.pageAnim = pageAnim;
    },
    changePrimaryColor(color: string) {
      this.themeColor = color;
      usePrimaryColor(color);
    },
    changeSideWidth(sideWidth: number) {
      this.sideWidth = sideWidth;
      useChangeMenuWidth(sideWidth);
    },
    toggleCollapse(isCollapse: boolean) {
      this.isCollapse = isCollapse;
    },
    setMainHeight(height: number) {
      this.mainHeight = height;
    },
    setFlexMainHeight(isFlex: boolean) {
      this.flexMainHeight = isFlex;
    },
  },
  persist: {
    enable: true,
    restoreState: true,
    option: {
      exclude: ['flexMainHeight'],
    },
  },
});

export default useAppConfigStore;
