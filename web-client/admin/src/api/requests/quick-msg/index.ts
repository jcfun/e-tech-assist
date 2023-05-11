import type { QueryQuickMsgDTO, QueryQuickMsgVO } from '../../types/quickMsg/index';
import http from '@/api';
import { Method, type PageRes, type Res } from '@/api/types/common';
class QuickMsg {
  // 多条件模糊查询
  public getQuickMsgsFq = (data: QueryQuickMsgDTO): Promise<Res<PageRes<Array<QueryQuickMsgVO>>>> => {
    return http.request({
      url: '/quick-msg/fq',
      method: Method.POST,
      data,
    });
  };

  public updateDisableFlag = (id: string, disableFlag = '0'): Promise<Res<null>> => {
    return http.request({
      url: `/quick-msg/${id}/${disableFlag}`,
      method: Method.PATCH,
    });
  };
}

const quickMsg = new QuickMsg();
export default quickMsg;
