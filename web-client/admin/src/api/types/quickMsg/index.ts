export interface QueryQuickMsgVO {
  id: string;
  operateTime: string;
  operator: string;
  operatorId: string;
  createTime: string;
  creator: string;
  creatorId: string;
  deleteFlag: string;
  senderId: string;
  recipientId: string;
  title: string;
  content: string;
  successFlag: string;
  sendMethod: string;
  description: string;
  msgType: string;
  replyId: string;
  readFlag: string;
  senderEmail: string;
  recipientEmail: string;
  senderAvatar: string;
  recipientAvatar: string;
  disableFlag: string;
  children: Array<QueryQuickMsgVO>;
}

export interface QueryQuickMsgDTO {
  createTimeStart: string;
  createTimeEnd: string;
  sender: string;
  recipient: string;
  title: string;
  sendMethod: string;
  msgType: string;
  readFlag: string;
  successFlag: string;
  disableFlag: string;
  pageNo: number;
  pageSize: number;
}
