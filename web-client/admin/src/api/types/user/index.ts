import type { RoleVO } from '../role';

export interface QueryUserDTO {
  createTimeStart?: string;
  createTimeEnd?: string;
  id?: string;
  nickname?: string;
  email?: string;
  phoneNumber?: string;
  gender?: string;
  disableFlag?: string;
  pageNo: number;
  pageSize: number;
}

export interface UserVO {
  id: string;
  operateTime: string;
  operator: string;
  operatorId: string;
  createTime: string;
  creator: string;
  creatorId: string;
  deleteFlag: string;
  account: string;
  disableFlag: string;
  detailId: string;
  description: string;
  openid: string;
  phoneNumber: string;
  email: string;
  nickname: string;
  avatarUrl: string;
  lastLoginTime: string;
  lastLoginIp: string;
  language: string;
  country: string;
  province: string;
  city: string;
  roles: Array<RoleVO>;
}

export interface UserDTO {
  [key: string]: string | Array<string> | undefined;
  id: string;
  account: string;
  password: string;
  disable_flag: string;
  roleIds?: Array<string>;
  description?: string;
  phoneNumber: string;
  nickname: string;
}
