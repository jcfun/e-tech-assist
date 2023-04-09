import type { ComReqParams } from './../models/common';
class Request {
  private baseUrl = 'http://192.168.31.201:3000/api/v1';

  public request<T extends object, U extends object>(comReqParams: ComReqParams<U>): Promise<T> {
    return new Promise((resolve, reject) => {
      console.log('request==================>', comReqParams);
      uni.request({
        url: `${this.baseUrl}${comReqParams.url}`,
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
