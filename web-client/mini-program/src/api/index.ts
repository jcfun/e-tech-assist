/* eslint-disable @typescript-eslint/no-explicit-any */
import type { ComReqParams, Res } from './types/common';
import { useUserStore } from '@/store/user';
class Request {
  private baseUrl = 'http://172.18.32.1:33000/api/v1';
  private baseUrl2 = 'http://10.4.6.5:33000/api/v1';
  private baseUrlWindows = 'http://192.168.31.201:3000/api/v1';
  private baseUrlLinux = 'http://10.7.7.2:33000/api/v1';

  private user = useUserStore();

  // private isWindows = navigator.platform.indexOf('Win') !== -1;
  // private isLinux = navigator.platform.indexOf('Linux') !== -1;

  public request<T extends object, U extends object>(comReqParams: ComReqParams<U>): Promise<T> {
    return new Promise((resolve, reject) => {
      console.log('request==================>', comReqParams);
      // if (this.isWindows) {
      //   this.baseUrl = this.baseUrlWindows;
      // } else if (this.isLinux) {
      //   this.baseUrl = this.baseUrlLinux;
      // }
      uni.request({
        url: `${this.baseUrl2}${comReqParams.url}`,
        data: comReqParams.data,
        method: comReqParams.method,
        header: {
          'content-type': 'application/json',
          Authorization: `${this.user.token.tokenType} ${this.user.token.token}`,
        },
        success: res => {
          console.log('res ===> ', res);
          if ((res.data as Res<any>).code != 200) {
            uni.showToast({
              title: (res.data as Res<any>).msg,
              icon: 'error',
              mask: true,
              duration: 2000,
            });
            if ((res.data as Res<any>).code == 401) {
              uni.reLaunch({
                url: '/pages/mine/mine',
              });
              return;
            }
          }
          return resolve(res.data as T);
        },
        fail: err => {
          console.log('err ===>', err);
          uni.showToast({
            title: err.errMsg,
            icon: 'error',
            mask: true,
            duration: 2000,
          });
          return reject(err.errMsg);
        },
      });
    });
  }
}

export default new Request();
