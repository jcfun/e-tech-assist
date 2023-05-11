/* eslint-disable @typescript-eslint/no-explicit-any */
import axios from 'axios';
import type { ComReqParams, Res } from './types/common';
import { Message } from '@arco-design/web-vue';
import { initInterceptors } from './interceptors';
import { npAction } from '@/styles';

export const service = axios.create({
  timeout: 60000,
});
initInterceptors(service);

// import { useUserStore } from '@/store/user';
class Http {
  private baseUrl = 'http://172.18.32.1:33000/api/v1';
  private baseUrl2 = 'http://10.4.6.4:33000/api/v1';
  private baseUrlWindows = 'http://192.168.31.201:3000/api/v1';
  private baseUrlLinux = 'http://10.7.7.2:33000/api/v1';
  private baseUrlLinux2 = 'http://ddns.urainstar.top:33000/api/v1';

  // private user = useUserStore();

  // private isWindows = navigator.platform.indexOf('Win') !== -1;
  // private isLinux = navigator.platform.indexOf('Linux') !== -1;

  public request<T extends object, U extends object>(comReqParams: ComReqParams<U>): Promise<T> {
    return new Promise((resolve, reject) => {
      console.log('request ==================> ', comReqParams);
      // if (this.isWindows) {
      //   this.baseUrl = this.baseUrlWindows;
      // } else if (this.isLinux) {
      //   this.baseUrl = this.baseUrlLinux;
      // }
      service
        .request({
          url: `${this.baseUrl}${comReqParams.url}`,
          method: comReqParams.method,
          data: comReqParams.data,
        })
        .then(res => {
          console.log('response ==================> ', res);
          if ((res.data as Res<T>).code != 200 && (res.data as Res<T>).code != 401) {
            Message.error({
              content: (res.data as Res<T>).msg,
              duration: 2000,
            });
          }
          return resolve(res.data as T);
        })
        .catch(err => {
          console.log('err ==========> ' + err);
          return reject(err.errMsg);
        })
        .finally(() => {
          npAction.done();
        });
    });
  }
}
const http = new Http();
export default http;
