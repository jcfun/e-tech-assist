import http from '@/api';
import { Method, type Res } from '@/api/types/common';
import type { Captcha, LoginDTO, LoginVO, RegisterDTO, ResetPwdDTO } from '@/api/types/login';

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

  public register = (data: RegisterDTO): Promise<Res<null>> => {
    return http.request({
      url: '/login/register',
      method: Method.POST,
      data,
    });
  };

  public resetPwd = (data: ResetPwdDTO): Promise<Res<null>> => {
    return http.request({
      url: '/login/reset-pwd',
      method: Method.PATCH,
      data,
    });
  };
}

const login = new Login();
export default login;
