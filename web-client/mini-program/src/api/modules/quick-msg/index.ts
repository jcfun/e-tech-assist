import request from '../..';
import { Method, type PageRes, type Res } from '@/api/types/common';
import type { CreateQuickMsgDTO, QueryQuickMsgVO, UpdateReadFlagDTO } from '@/api/types/quick-msg';

class QuickMsg {
  public sendQuickMsg = (data: CreateQuickMsgDTO): Promise<Res<null>> => {
    return request.request({
      url: '/quick-msg',
      method: Method.POST,
      data,
    });
  };

  public getQuickMsgList = (pageNo: number, pageSize: number): Promise<Res<PageRes<QueryQuickMsgVO>>> => {
    return request.request({
      url: `/quick-msg/${pageNo}/${pageSize}`,
      method: Method.GET,
    });
  };

  public updateReadFlag(data: UpdateReadFlagDTO): Promise<Res<number>> {
    console.log('data ===> ', data);
    return request.request({
      url: `/quick-msg`,
      method: Method.PUT,
      data,
    });
  }

  public getQuickMsgReplyList(id: string): Promise<Res<PageRes<QueryQuickMsgVO>>> {
    return request.request({
      url: `/quick-msg/${id}`,
      method: Method.GET,
    });
  }
}

const quickMsg = new QuickMsg();
export default quickMsg;
