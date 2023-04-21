import type { RoleVO } from '../role';

export interface Captcha {
  uuid: string;
  img: string;
}

export interface LoginDTO {
  identity: string;
  password: string;
  captcha: string;
  uuid: string;
}

export interface LoginVO {
  token: Token;
  userInfo: UserInfo;
}

export interface Token {
  token: string;
  tokenType: string;
}

export interface UserInfo {
  account: string;
  avatarUrl: string;
  city: string;
  country: string;
  createTime: string;
  creator: string;
  creatorId: string;
  deleteFlag: string;
  description: string;
  detailId: string;
  disableFlag: string;
  email: string;
  gender: string;
  id: string;
  language: string;
  lastLoginIp: string;
  lastLoginTime: string;
  nickname: string;
  openid: string;
  operateTime: string;
  operator: string;
  operatorId: string;
  phoneNumber: string;
  province: string;
  roles: Array<RoleVO>;
}
