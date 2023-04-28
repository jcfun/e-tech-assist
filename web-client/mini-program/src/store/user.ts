import { defineStore } from 'pinia';
import { Names } from './store-name';
import type { Token, UserInfo } from '@/api/types/login/login';

export const useUserStore = defineStore(Names.USER, {
  state: () => {
    return {
      token: <Token>{},
      userInfo: <UserInfo>{},
    };
  },
  getters: {},
  actions: {
    setToken(token: Token) {
      this.token = token;
    },
    setUserInfo(userInfo: UserInfo) {
      this.userInfo = userInfo;
    },
  },
});
