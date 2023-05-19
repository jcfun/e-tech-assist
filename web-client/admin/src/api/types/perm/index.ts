export interface PermVO {
  apiPath: string;
  createTime: string;
  creator: string;
  creatorId: string;
  deleteFlag: string;
  description: string;
  disableFlag: string;
  code: string;
  route: string;
  routeName: string;
  id: string;
  name: string;
  operateTime: string;
  operator: string;
  operatorId: string;
  parentId: string;
  permType: string;
  resource: string;
  hiddenFlag: string;
  parentRoute: string;
  children: Array<PermVO>;
}

export interface CreatePermDTO {
  name: string;
  parentId: string;
  permType: string;
  disableFlag: string;
  apiPath: string;
  route: string;
  routeName: string;
  code: string;
  resource: string;
  hiddenFlag: string;
  parentRoute: string;
  description: string;
}

export interface UpdatePermDTO {
  id: string;
  name: string;
  parentId: string;
  permType: string;
  disableFlag: string;
  apiPath: string;
  route: string;
  routeName: string;
  code: string;
  resource: string;
  hiddenFlag: string;
  parentRoute: string;
  description: string;
}

export interface QueryPermDTO {
  createTimeStart: string;
  createTimeEnd: string;
  name: string;
  permType: string;
  disableFlag: string;
  hiddenFlag: string;
  pageNo: number;
  pageSize: number;
}
