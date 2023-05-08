import type { PermVO } from '../perm';

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
  perms: Array<PermVO>;
}

export interface QueryLoginLogDTO {
  createTimeStart: string;
  createTimeEnd: string;
  identity: string;
  successFlag: string;
  method: string;
  pageNo: number;
  pageSize: number;
}

export interface QueryLoginLogVO {
  id: string;
  operateTime: string;
  operator: string;
  operatorId: string;
  createTime: string;
  creator: string;
  creatorId: string;
  deleteFlag: string;
  account: string;
  successFlag: string;
  description: string;
  userAgent: string;
  ip: string;
  location: string;
  mac: string;
  method: string;
}
