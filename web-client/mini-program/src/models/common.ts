export interface ComReqParams<T extends object> {
  url: string;
  data: T;
  method: Method;
}

export enum Method {
  OPTIONS = 'OPTIONS',
  GET = 'GET',
  HEAD = 'HEAD',
  POST = 'POST',
  PUT = 'PUT',
  DELETE = 'DELETE',
  TRACE = 'TRACE',
  CONNECT = 'CONNECT',
}

export interface Res<T> {
  code: string;
  msg: string;
  data: T;
}
