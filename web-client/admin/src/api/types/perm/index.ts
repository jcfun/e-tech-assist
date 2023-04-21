export interface PermVO {
  apiPath: string;
  createTime: string;
  creator: string;
  creatorId: string;
  deleteFlag: string;
  description: string;
  disableFlag: string;
  feCode: string;
  feRoute: string;
  feName: string;
  id: string;
  name: string;
  operateTime: string;
  operator: string;
  operatorId: string;
  parentId: string;
  permType: string;
  resource: string;
  children: Array<PermVO>;
}
