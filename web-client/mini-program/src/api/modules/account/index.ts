import request from '../..';
import { Method, type Res } from '@/api/types/common';
import type { AccountInfo } from '@/api/types/account';

export const saveAccount = (account: AccountInfo): Promise<Res<number>> => {
  return request.request({
    url: '/user/wxapp',
    method: Method.PUT,
    data: account,
  });
};
