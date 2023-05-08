import type { AxiosResponse } from 'axios';
import { Message } from '@arco-design/web-vue';
import useUserStore from '@/store/modules/user';

export default function (response: AxiosResponse): AxiosResponse {
  if (response.data.code === 401) {
    Message.error('当前用户登录已过期，请重新登录');
    setTimeout(() => {
      useUserStore()
        .logout()
        .then(() => {
          window.location.hash = '#/login?redirect=/';
          window.location.reload();
        });
    }, 1500);
  }
  return response;
}
