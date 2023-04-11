import request from './request';
import { Method, type Res } from '@/models/common';
import type { Account } from '@/models/account';

export const saveAccount = (account: Account): Promise<Res<number>> => {
  return request.request({
    url: '/user/wxapp',
    method: Method.PUT,
    data: account,
  });
};
