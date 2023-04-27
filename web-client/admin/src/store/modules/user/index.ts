import type { LoginVO } from '@/api/types/login';
import { Names } from '@/store/types/store-name';
import { defineStore } from 'pinia';

const useUserStore = defineStore(Names.USER, {
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
    tokenExpireFlag() {
      return !this.user?.token?.token;
    },
  },
  persist: {
    enable: true,
    restoreState: true,
  },
});
export default useUserStore;
