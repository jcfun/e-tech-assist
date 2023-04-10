import type { ComReqParams } from './../models/common';
class Request {
  private baseUrl = 'http://192.168.31.201:3000/api/v1';
  private baseUrlWindows = 'http://192.168.31.201:3000/api/v1';
  private baseUrlLinux = 'http://10.7.7.5:33000/api/v1';

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
        url: `${this.baseUrlLinux}${comReqParams.url}`,
        data: comReqParams.data,
        method: comReqParams.method,
        header: {
          'content-type': 'application/json',
        },
        success: res => {
          console.log('res ===> ', res);
          return resolve(res.data as T);
        },
        fail: err => {
          console.log('err ===>', err);
          return reject(err.errMsg);
        },
      });
    });
  }
}

export default new Request();
