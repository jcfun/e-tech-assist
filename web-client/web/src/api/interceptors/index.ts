import { useUserStore } from '@/stores/modules/user';
import { npAction } from '@/styles';
import type { AxiosInstance } from 'axios';

const userStore = useUserStore();

export const initInterceptors = (instance: AxiosInstance) => {
  // 添加请求拦截器
  instance.interceptors.request.use(
    config => {
      npAction.done();
      npAction.start();
      config.headers['Authorization'] = userStore?.user?.token?.tokenType + ' ' + userStore?.user?.token?.token;
      console.log('请求拦截器....');
      // 在发送请求之前做些什么
      return config;
    },
    function (error) {
      // 对请求错误做些什么
      return Promise.reject(error);
    },
  );

  // 添加响应拦截器
  instance.interceptors.response.use(
    function (response) {
      console.log('响应拦截器....');
      // 2xx 范围内的状态码都会触发该函数。
      // 对响应数据做点什么
      return response;
    },
    function (error) {
      // 超出 2xx 范围的状态码都会触发该函数。
      // 对响应错误做点什么
      return Promise.reject(error);
    },
  );
};
