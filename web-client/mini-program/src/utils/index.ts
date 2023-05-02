// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const isEmpty = (value: any): boolean => {
  if (value === null || value === undefined || value === '') {
    return true;
  }
  return false;
};
