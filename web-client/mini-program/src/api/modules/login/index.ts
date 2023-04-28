import request from '../..';
import { Method, type Res } from '@/api/types/common';
import type { LoginDTO, LoginVO, RegisterDTO } from '@/api/types/login';

export const login = (data: LoginDTO): Promise<Res<LoginVO>> => {
  return request.request({
    url: '/login/wxapp',
    method: Method.POST,
    data: data,
  });
};

export const register = (data: RegisterDTO): Promise<Res<LoginVO>> => {
  return request.request({
    url: '/login/wr',
    method: Method.POST,
    data: data,
  });
};
