import { createPinia, type PiniaPluginContext } from 'pinia';
import { toRaw } from 'vue';

interface Options {
  prefix: string;
  expire: number;
}

interface Storage {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  data: any;
  expire: number;
}

const setStorage = (key: string, value: Storage) => {
  uni.setStorageSync(key, JSON.stringify(value));
};

const getStorage = (key: string) => {
  // console.log('store =====> ', uni.getStorageSync(key));
  const value = uni.getStorageSync(key);
  if (value) {
    if (value.expire < Date.now()) {
      console.log('storage已过期, removed');
      uni.removeStorageSync(key);
      return {};
    } else {
      if (typeof value === 'string') {
        return <Storage>JSON.parse(value).data;
      }
    }
  } else {
    return {};
  }
};

const piniaPlugin = (options: Options) => {
  return (context: PiniaPluginContext) => {
    const { store } = context;
    store.$subscribe(() => {
      setStorage(`${options.prefix}-${store.$id}`, <Storage>{
        data: toRaw(store.$state),
        expire: options.expire,
      });
    });
    return getStorage(`${options.prefix}-${store.$id}`);
  };
};

const store = createPinia();
store.use(
  piniaPlugin({
    prefix: 'pinia',
    expire: Date.now() + 1000 * 60 * 60 * 24 * 7,
  }),
);

export default store;
