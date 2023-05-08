import type { CreatePermDTO, PermVO, QueryPermDTO, UpdatePermDTO } from './../../types/perm/index';
import http from '@/api';
import { Method, type PageRes, type Res } from '@/api/types/common';
class Perm {
  // 多条件模糊查询
  public getPermsFq = (data: QueryPermDTO): Promise<Res<PageRes<Array<PermVO>>>> => {
    return http.request({
      url: '/perm/fq',
      method: Method.POST,
      data,
    });
  };

  public createPerm = (data: CreatePermDTO): Promise<Res<null>> => {
    return http.request({
      url: '/perm',
      method: Method.POST,
      data,
    });
  };

  public updatePerm = (data: UpdatePermDTO): Promise<Res<null>> => {
    return http.request({
      url: '/perm',
      method: Method.PUT,
      data,
    });
  };

  public deletePerm = (id: string): Promise<Res<null>> => {
    return http.request({
      url: `/perm/${id}`,
      method: Method.DELETE,
    });
  };

  public updateDisableFlag = (id: string, disableFlag = '0'): Promise<Res<null>> => {
    return http.request({
      url: `/perm/${id}/${disableFlag}`,
      method: Method.PATCH,
    });
  };

  // 全量查询
  public getPerms = (): Promise<Res<PageRes<PermVO>>> => {
    return http.request({
      url: '/perm',
      method: Method.GET,
    });
  };
}

const perm = new Perm();
export default perm;
