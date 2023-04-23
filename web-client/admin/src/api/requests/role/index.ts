import http from '@/api';
import { Method, type PageRes, type Res } from '@/api/types/common';
import type { RoleVO } from '@/api/types/role';
class Role {
  // public getUsersFq = (data: QueryUserDTO): Promise<Res<PageRes<Array<UserVO>>>> => {
  //   return http.request({
  //     url: '/user/fq',
  //     method: Method.POST,
  //     data,
  //   });
  // };

  // public createUser = (data: CreateUserDTO): Promise<Res<string>> => {
  //   return http.request({
  //     url: '/user',
  //     method: Method.POST,
  //     data,
  //   });
  // };

  public getRole = (): Promise<Res<PageRes<RoleVO>>> => {
    return http.request({
      url: '/role',
      method: Method.GET,
    });
  };
}

const role = new Role();
export default role;
