import request from './request';
import { Method, type Res } from '@/models/common';
import type { LoginDTO, LoginVO, RegisterDTO } from '@/models/login';

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
