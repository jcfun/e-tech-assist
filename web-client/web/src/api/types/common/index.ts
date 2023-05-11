export interface ComReqParams<T extends object> {
  url: string;
  method: Method;
  data?: T;
}

export enum Method {
  OPTIONS = 'OPTIONS',
  GET = 'GET',
  HEAD = 'HEAD',
  POST = 'POST',
  PUT = 'PUT',
  DELETE = 'DELETE',
  PATCH = 'PATCH',
  TRACE = 'TRACE',
  CONNECT = 'CONNECT',
}

export interface Res<T> {
  code: number;
  msg: string;
  data: T;
}

export interface PageRes<T> {
  data: Array<T>;
  total: number;
  total_page: number;
  current_page: number;
}
