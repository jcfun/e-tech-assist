import { createPinia, type PiniaPluginContext } from 'pinia';
import { toRaw, type App } from 'vue';

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
  localStorage.setItem(key, JSON.stringify(value));
};

const getStorage = (key: string) => {
  // console.log('store =====> ', uni.getStorageSync(key));
  const value: Storage = JSON.parse(localStorage.getItem(key) || '{}');
  if (value) {
    if (value.expire < Date.now()) {
      console.log('storage已过期, removed');
      localStorage.removeItem(key);
      return {};
    } else {
      return value.data;
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

export const initStores = (app: App) => {
  app.use(store);
};

export default store;
