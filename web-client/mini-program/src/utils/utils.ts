import { AUTH_BASE_URL } from '../api/request';
export const toastDuration = 2000;
export const needDealHttpCode = {
  400: '错误请求',
  401: '未授权',
  403: '禁止',
  404: '未找到',
  405: '方法禁用',
  500: '服务器内部错误',
  502: '错误网关',
  503: '服务不可用',
  504: '网关超时',
};

export function isMobile(v) {
  return /^1[3-9]\d{9}$/.test(v);
}
export function isVerCode(v) {
  return /^[0-9]{6}$/.test(v);
}

export function getParams(params) {
  const paramsArray = [];
  Object.entries(params)
    .filter(item => item[1] != null && item[1] != undefined)
    .forEach((value, index, _arr) => {
      paramsArray.push(index === 0 ? `?${value[0]}=${value[1]}` : `${value[0]}=${value[1]}`);
    });
  return paramsArray;
}

export function getBindOfficialAccountUrl(officialAccountAppId, phoneNumber) {
  return 'https://open.weixin.qq.com/connect/oauth2/authorize?appid=' + officialAccountAppId + '&redirect_uri=' + encodeURIComponent(AUTH_BASE_URL + '/midea-platform/bind-mp-user') + '&response_type=code&scope=snsapi_base&state=' + phoneNumber;
}

//格式化为时间
export function formatTime(value) {
  if (value) {
    const date = new Date(value);
    const y = date.getFullYear();
    let MM = date.getMonth() + 1;
    MM = MM < 10 ? '0' + MM : MM;
    let d = date.getDate();
    d = d < 10 ? '0' + d : d;
    let h = date.getHours();
    h = h < 10 ? '0' + h : h;
    let m = date.getMinutes();
    m = m < 10 ? '0' + m : m;
    let s = date.getSeconds();
    s = s < 10 ? '0' + s : s;
    if (isNaN(y) || isNaN(MM)) {
      return value;
    } else {
      return y + '-' + MM + '-' + d + ' ' + h + ':' + m + ':' + s;
    }
  } else {
    return ' ';
  }
}
//格式化为日期
export function formatDate(value) {
  if (value) {
    const date = new Date(value);
    const y = date.getFullYear();
    let MM = date.getMonth() + 1;
    MM = MM < 10 ? '0' + MM : MM;
    let d = date.getDate();
    d = d < 10 ? '0' + d : d;
    return y + '-' + MM + '-' + d;
  } else {
    return '';
  }
}
export function getPreMonth(date) {
  const arr = date.split('-');
  const year = arr[0]; //获取当前日期的年份
  const month = arr[1]; //获取当前日期的月份
  const day = arr[2]; //获取当前日期的日
  // let days = new Date(year, month, 0);
  // days = days.getDate(); //获取当前日期中月的天数
  let year2 = year;
  let month2 = parseInt(month) - 1;
  if (month2 == 0) {
    year2 = parseInt(year2) - 1;
    month2 = 12;
  }
  let day2 = day;
  let days2 = new Date(year2, month2, 0);
  days2 = days2.getDate();
  if (day2 > days2) {
    day2 = days2;
  }
  if (month2 < 10) {
    month2 = '0' + month2;
  }
  const t2 = year2 + '-' + month2 + '-' + day2;
  return t2;
}
//格式化为日期截至到月份
export function formatMonth(value) {
  if (value) {
    const date = new Date(value);
    const y = date.getFullYear();
    let MM = date.getMonth() + 1;
    MM = MM < 10 ? '0' + MM : MM;
    // let d = date.getDate();
    // d = d < 10 ? '0' + d : d;
    return y + '-' + MM;
  } else {
    return '';
  }
}
/** 是否外部链接
 * @param {string} path
 * @returns {Boolean}
 */
export function isExternal(path) {
  return /^(https?:|mailto:|tel:)/.test(path);
}

/** 是否网络链接
 * @param {string} url
 * @returns {Boolean}
 */
export function isHttpURL(url) {
  return /(http|https):\/\/([\w.]+\/?)\S*/.test(url);
}

/** 是否url
 * @param {string} url
 * @returns {Boolean}
 */
export function validURL(url) {
  const reg =
    /^(https?|ftp):\/\/([a-zA-Z0-9.-]+(:[a-zA-Z0-9.&%$-]+)*@)*((25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9][0-9]?)(\.(25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])){3}|([a-zA-Z0-9-]+\.)*[a-zA-Z0-9-]+\.(com|edu|gov|int|mil|net|org|biz|arpa|info|name|pro|aero|coop|museum|[a-zA-Z]{2}))(:[0-9]+)*(\/($|[a-zA-Z0-9.,?'\\+&%$#=~_-]+))*$/;
  return reg.test(url);
}

/** 是否小写字符串
 * @param {string} str
 * @returns {Boolean}
 */
export function validLowerCase(str) {
  const reg = /^[a-z]+$/;
  return reg.test(str);
}

/** 是否大写字符串
 * @param {string} str
 * @returns {Boolean}
 */
export function validUpperCase(str) {
  const reg = /^[A-Z]+$/;
  return reg.test(str);
}

/** 是否字母
 * @param {string} str
 * @returns {Boolean}
 */
export function validAlphabets(str) {
  const reg = /^[A-Za-z]+$/;
  return reg.test(str);
}

/** 是否有效邮箱
 * @param {string} email
 * @returns {Boolean}
 */
export function validEmail(email) {
  const reg = '/^(([^<>()[]\\.,;:s@"]+(.[^<>()[]\\.,;:s@"]+)*)|(".+"))@(([[0-9]{1,3}.[0-9]{1,3}.[0-9]{1,3}.[0-9]{1,3}])|(([a-zA-Z-0-9]+.)+[a-zA-Z]{2,}))$/';
  return reg.test(email);
}

/** 是否字符串
 * @param {string} str
 * @returns {Boolean}
 */
export function isString(str) {
  if (typeof str === 'string' || str instanceof String) {
    return true;
  }
  return false;
}

/** 是否数组
 * @param {Array} arg
 * @returns {Boolean}
 */
export function isArray(arg) {
  if (typeof Array.isArray === 'undefined') {
    return Object.prototype.toString.call(arg) === '[object Array]';
  }
  return Array.isArray(arg);
}

/** 是否有效身份证
 * @param {string} idcard
 * @returns {Boolean}
 */
export function validIdCard(idcard) {
  const reg = /(^\d{15}$)|(^\d{18}$)|(^\d{17}(\d|X|x)$)/;
  return reg.test(idcard);
}

/** 是否有效数字
 * @param {string} input
 * @returns {Boolean}
 */
export function isNumber(input) {
  const reg = /(^(-?\d+)(\.\d+)?$)/;
  return reg.test(input);
}

/** 是否有效手机号码
 * @param {string} input
 * @returns {Boolean}
 */
export function isMobilePhone(input) {
  const reg = /^(13[0-9]|14[01456879]|15[0-3,5-9]|16[2567]|17[0-8]|18[0-9]|19[0-3,5-9])\d{8}$/;
  return reg.test(input);
}

export function havePermission(permissionList = []) {
  const permissionMap = getApp().globalData.grantedPolicies;
  let count = 0;
  permissionList.forEach(permission => {
    if (permissionMap[permission]) {
      count++;
    }
  });
  return count == permissionList.length;
}
// 保存图片
export function savePicture(tempFilePath) {
  uni.getSetting({
    success(res) {
      console.log(res.authSetting);
      if (!res.authSetting['scope.writePhotosAlbum']) {
        //向用户发起授权请求
        uni.authorize({
          scope: 'scope.writePhotosAlbum',
          success: () => {
            uni.saveImageToPhotosAlbum({
              filePath: tempFilePath,
              success: function () {
                uni.showToast({
                  icon: 'none',
                  title: '图片已下载至【图库】，请打开【图库】查看',
                });
              },
            });
          },
          fail: () => {
            uni.showModal({
              title: '您已拒绝获取相册权限',
              content: '是否进入权限管理，调整授权？',
              success: res => {
                if (res.confirm) {
                  //调起客户端小程序设置界面，返回用户设置的操作结果。（重新让用户授权）
                  uni.openSetting({
                    success: res => {
                      console.log(res.authSetting);
                    },
                  });
                }
              },
            });
          },
        });
      } else {
        uni.saveImageToPhotosAlbum({
          filePath: tempFilePath,
          success: function () {
            uni.showToast({
              icon: 'none',
              title: '图片已下载至【图库】，请打开【图库】查看',
            });
          },
        });
      }
    },
  });
}
// 获取地址权限
export function getAddress() {
  return new Promise(function (resolve, _reject) {
    uni.getSetting({
      success(res) {
        console.log(res.authSetting);
        if (!res.authSetting['scope.address']) {
          //向用户发起授权请求
          uni.authorize({
            scope: 'scope.address',
            success: () => {
              uni.chooseAddress({
                success(res) {
                  resolve(res);
                },
              });
            },
            fail: () => {
              uni.showModal({
                title: '您已拒绝获取地址权限',
                content: '是否进入权限管理，调整授权？',
                success: res => {
                  if (res.confirm) {
                    //调起客户端小程序设置界面，返回用户设置的操作结果。（重新让用户授权）
                    uni.openSetting({
                      success: res => {
                        console.log(res.authSetting);
                      },
                    });
                  }
                },
              });
            },
          });
        } else {
          uni.chooseAddress({
            success(res) {
              resolve(res);
            },
          });
        }
      },
    });
  });
}
/**
 * 临时图片路径转base64
 * url: 临时图片存放路径，选择图片返回的相对路径
 * type: 图片类型，如：png
 * @param url
 * @param type
 */
export function base64H(url, type) {
  return new Promise((resolve, reject) => {
    if (uni.getFileSystemManager()) {
      uni.getFileSystemManager().readFile({
        filePath: url, //选择图片返回的相对路径
        encoding: 'base64', //编码格式
        success: res => {
          resolve('data:image/' + type.toLocaleLowerCase() + ';base64,' + res.data);
        },
        fail: res => reject(res.errMsg),
      });
    } else {
      uni.request({
        url: url,
        method: 'GET',
        responseType: 'arraybuffer',
        success: ress => {
          let base64 = wx.arrayBufferToBase64(ress.data); //把arraybuffer转成base64
          base64 = 'data:image/jpeg;base64,' + base64; //不加上这串字符，在页面无法显示的哦
          resolve(base64);
        },
        fail: res => reject(res.errMsg),
      });
    }
  });
}
export function isNeedDealErrorCode(httpCode) {
  return Object.keys(needDealHttpCode).find(item => item == httpCode);
}

export function dealErrorcode(code, httpCode, options, data) {
  if (code == 'TOKENFORBIDNULL' || httpCode == 401) {
    uni.showToast({
      mask: true,
      icon: 'none',
      duration: toastDuration,
      title: '登录失效，即将重登录！',
    });
    uni.removeStorageSync('accessToken');
    setTimeout(() => {
      uni.redirectTo({ url: '/pages/tabBar/mine' });
    }, toastDuration);
  } else if (httpCode == 403) {
    uni.showToast({
      mask: true,
      icon: 'none',
      duration: toastDuration,
      title: data ? (data.error && data.error.message ? data.error.message : JSON.stringify(data.error || data)) : needDealHttpCode[httpCode] + '：' + options.url + '！',
    });
  } else {
    uni.showToast({
      mask: true,
      icon: 'none',
      duration: toastDuration,
      title: data ? (data.error && data.error.message ? data.error.message : JSON.stringify(data.error || data)) : needDealHttpCode[httpCode] + '！' + '(异常编码：' + httpCode + ')',
    });
  }
}
