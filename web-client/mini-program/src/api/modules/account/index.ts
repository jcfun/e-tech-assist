import request from '../..';
import { Method, type Res } from '@/api/types/common';
import type { Account } from '@/api/types/account';

export const saveAccount = (account: Account): Promise<Res<number>> => {
  return request.request({
    url: '/user/wxapp',
    method: Method.PUT,
    data: account,
  });
};
