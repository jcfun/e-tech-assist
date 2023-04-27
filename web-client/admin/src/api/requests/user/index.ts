import http from '@/api';
import { Method, type PageRes, type Res } from '@/api/types/common';
import type { QueryUserDTO, UserVO, UpdateUserDTO, CreateUserDTO } from '@/api/types/user';
class User {
  public getUsersFq = (data: QueryUserDTO): Promise<Res<PageRes<Array<UserVO>>>> => {
    return http.request({
      url: '/user/fq',
      method: Method.POST,
      data,
    });
  };

  public createUser = (data: CreateUserDTO): Promise<Res<null>> => {
    return http.request({
      url: '/user',
      method: Method.POST,
      data,
    });
  };

  public updateUser = (data: UpdateUserDTO): Promise<Res<null>> => {
    return http.request({
      url: '/user',
      method: Method.PUT,
      data,
    });
  };

  public deleteUser = (id: string): Promise<Res<null>> => {
    return http.request({
      url: `/user/${id}`,
      method: Method.DELETE,
    });
  };

  public updateUserStatus = (id: string, disableFlag = '0'): Promise<Res<null>> => {
    return http.request({
      url: `/user/${id}/${disableFlag}`,
      method: Method.PATCH,
    });
  };
}

const user = new User();
export default user;
