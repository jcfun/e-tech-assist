import useUserStore from '@/store/modules/user';
import type { InternalAxiosRequestConfig } from 'axios';

export default function (config: InternalAxiosRequestConfig) {
  const useStore = useUserStore();
  if (config) {
    if (!config.headers) {
      config.headers = {} as any;
    }
    if (!config.headers['Authorization']) {
      config.headers['Authorization'] = `${useStore?.user?.token?.tokenType} ${useStore?.user?.token?.token}`;
    }
  }
  return config;
}
