import http from '@/api';
import { Method, type Res } from '@/api/types/common';
import type { Captcha, LoginDTO, LoginVO } from '@/api/types/login';

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
}

const login = new Login();
export default login;
