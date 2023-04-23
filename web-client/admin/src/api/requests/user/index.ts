import http from '@/api';
import { Method, type PageRes, type Res } from '@/api/types/common';
import type { UserDTO, QueryUserDTO, UserVO } from '@/api/types/user';
class User {
  public getUsersFq = (data: QueryUserDTO): Promise<Res<PageRes<Array<UserVO>>>> => {
    return http.request({
      url: '/user/fq',
      method: Method.POST,
      data,
    });
  };

  public createUser = (data: UserDTO): Promise<Res<string>> => {
    return http.request({
      url: '/user',
      method: Method.POST,
      data,
    });
  };

  public updateUser = (data: UserDTO): Promise<Res<string>> => {
    return http.request({
      url: '/user',
      method: Method.PUT,
      data,
    });
  };
}

const user = new User();
export default user;
