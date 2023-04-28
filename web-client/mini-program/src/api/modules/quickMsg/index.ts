import request from '../..';
import { Method, type Res } from '@/api/types/common';
import type { CreateQuickMsgDTO } from '@/api/types/quickMsg';

class QuickMsg {
  public sendQuickMsg = (data: CreateQuickMsgDTO): Promise<Res<null>> => {
    return request.request({
      url: '/quickMsg',
      method: Method.POST,
      data,
    });
  };
}

const quickMsg = new QuickMsg();
export default quickMsg;
