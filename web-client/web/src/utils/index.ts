const isEmpty = (value: string) => {
  return value === null || value === undefined || value === '';
};

import dayjs from 'dayjs';
type Time = string | Date | dayjs.Dayjs;
const timeInterval = (start: Time, end: Time): string => {
  return dayjs(end).diff(dayjs(start), 'minute') < 60
    ? `${dayjs(end).diff(dayjs(start), 'minute')}分钟前`
    : dayjs(end).diff(dayjs(start), 'hour') < 24
    ? `${dayjs(end).diff(dayjs(start), 'hour')}小时前`
    : `${dayjs(end).diff(dayjs(start), 'day')}天前`;
};

export { isEmpty, timeInterval };
