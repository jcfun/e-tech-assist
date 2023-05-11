import { defineStore } from 'pinia';
import { Names } from '../../types/store-name';
import type { LoginVO } from '@/api/types/login';

export const useUserStore = defineStore(Names.USER, {
  state: () => {
    return {
      user: <LoginVO>{},
    };
  },
  getters: {},
  actions: {
    setUser(user: LoginVO) {
      return new Promise<LoginVO>(resolve => {
        this.user = user;
        resolve(user);
      });
    },
    logout() {
      return new Promise<void>(resolve => {
        this.$reset();
        localStorage.clear();
        sessionStorage.clear();
        resolve();
      });
    },
  },
});
