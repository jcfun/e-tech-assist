import http from '@/api';
import { Method, type PageRes, type Res } from '@/api/types/common';
import type { CreateRoleDTO, QueryRoleDTO, RoleVO, UpdateRoleDTO, UpdateRolePermDTO } from '@/api/types/role';
class Role {
  // 多条件模糊查询
  public getRolesFq = (data: QueryRoleDTO): Promise<Res<PageRes<Array<RoleVO>>>> => {
    return http.request({
      url: '/role/fq',
      method: Method.POST,
      data,
    });
  };

  public createRole = (data: CreateRoleDTO): Promise<Res<null>> => {
    return http.request({
      url: '/role',
      method: Method.POST,
      data,
    });
  };

  public updateRole = (data: UpdateRoleDTO): Promise<Res<null>> => {
    return http.request({
      url: '/role',
      method: Method.PUT,
      data,
    });
  };

  public updateRolePerm = (data: UpdateRolePermDTO): Promise<Res<null>> => {
    return http.request({
      url: '/role/rp',
      method: Method.PUT,
      data,
    });
  };

  public deleteRole = (id: string): Promise<Res<null>> => {
    return http.request({
      url: `/role/${id}`,
      method: Method.DELETE,
    });
  };

  public updateRoleStatus = (id: string, disableFlag = '0'): Promise<Res<null>> => {
    return http.request({
      url: `/role/${id}/${disableFlag}`,
      method: Method.PATCH,
    });
  };

  // 全量查询
  public getRoles = (): Promise<Res<PageRes<RoleVO>>> => {
    return http.request({
      url: '/role',
      method: Method.GET,
    });
  };
}

const role = new Role();
export default role;
