export interface CreateQuickMsgDTO {
  senderId: string;
  recipientIdentity: string;
  title: string;
  content: string;
  sendType: string;
  msgType: string;
  replyId: string | null;
}

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
  failFlag: string;
  sendType: string;
  description: string;
  msgType: string;
  replyId: string;
  readFlag: string;
  senderEmail: string;
  recipientEmail: string;
  senderAvatar: string;
  recipientAvatar: string;
  replyQuickMsg: Array<QueryQuickMsgVO>;
}

export interface UpdateReadFlagDTO {
  ids: Array<string>;
  readFlag: string;
}
