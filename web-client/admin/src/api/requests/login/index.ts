import http from '@/api';
import { Method, type PageRes, type Res } from '@/api/types/common';
import type { Captcha, LoginDTO, LoginVO, QueryLoginLogDTO, QueryLoginLogVO } from '@/api/types/login';

class Login {
  public captcha = (): Promise<Res<Captcha>> => {
    return http.request({
      url: '/login/captcha',
      method: Method.GET,
    });
  };

  public login = (data: LoginDTO): Promise<Res<LoginVO>> => {
    return http.request({
      url: '/login',
      method: Method.POST,
      data,
    });
  };

  public getLoginLogsFq = (data: QueryLoginLogDTO): Promise<Res<PageRes<Array<QueryLoginLogVO>>>> => {
    return http.request({
      url: '/login/log/fq',
      method: Method.POST,
      data,
    });
  };
}

const login = new Login();
export default login;
