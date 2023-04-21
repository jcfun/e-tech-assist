export interface ComReqParams<T> {
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
  TRACE = 'TRACE',
  CONNECT = 'CONNECT',
}

export interface Res<T> {
  code: number;
  msg: string;
  data: T;
}
