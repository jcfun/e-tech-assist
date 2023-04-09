interface LoginDTO {
  code: string;
}

interface RegisterDTO {
  sessionKey: string;
  encryptedData: string;
  iv: string;
  openid: string;
  phoneNumber: string;
}

interface LoginVO {
  userInfo: UserInfo;
  token: Token;
}

interface UserInfo {
  id: string;
  operateTime: string;
  operator: string;
  operatorId: string;
  createTime: string;
  creator: string;
  creatorId: string;
  deleteFlag: string;
  account: string;
  disableFlag: string;
  detailId: string;
  roleId: string;
  description: string;
  phoneNumber: string;
  email: string;
  nickname: string;
  avatarUrl: string;
  lastLoginTime: string;
  lastLoginIp: string;
  gender: string;
  language: string;
  country: string;
  province: string;
  city: string;
  openid: string;
  sessionKey: string;
  encryptedData: string;
}

interface Token {
  token: string;
  tokenType: string;
}

export type { LoginDTO, LoginVO, RegisterDTO, UserInfo, Token };
