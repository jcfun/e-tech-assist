import type { PermVO } from '../perm';

export interface RoleVO {
  ode: string;
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
