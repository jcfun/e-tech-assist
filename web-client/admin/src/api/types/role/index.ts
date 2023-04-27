import type { PermVO } from '../perm';

export interface RoleVO {
  code: string;
  createTime: string;
  creator: string;
  creatorId: string;
  deleteFlag: string;
  description: string;
  disableFlag: string;
  id: string;
  name: string;
  operateTime: string;
  operator: string;
  operatorId: string;
  perms: Array<PermVO>;
}

export interface CreateRoleDTO {
  name: string;
  disableFlag: string;
  description: string;
  code: string;
  permIds: Array<string>;
}

export interface UpdateRoleDTO {
  id: string;
  name: string;
  disableFlag: string;
  description: string;
  code: string;
}

export interface UpdateRolePermDTO {
  id: string;
  permIds: Array<string>;
}

export interface QueryRoleDTO {
  createTimeStart: string;
  createTimeEnd: string;
  disableFlag: string;
  pageNo: number;
  pageSize: number;
}
