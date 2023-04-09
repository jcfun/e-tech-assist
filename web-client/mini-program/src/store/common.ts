import { createPinia, type PiniaPluginContext } from 'pinia';
import { toRaw } from 'vue';
// eslint-disable-next-line @typescript-eslint/no-explicit-any
const setStorage = (key: string, value: any) => {
  uni.setStorageSync(key, value);
};

const getStorage = (key: string) => {
  console.log('store =====> ', uni.getStorageSync(key));
  return uni.getStorageSync(key) ? JSON.parse(uni.getStorageSync(key)) : {};
};

interface Options {
  key: string;
}

const piniaPlugin = (options: Options) => {
  return (context: PiniaPluginContext) => {
    const { store } = context;
    const data = getStorage(`${options.key}-${store.$id}`);
    store.$subscribe(() => {
      setStorage(`${options.key}-${store.$id}`, toRaw(store.$state));
    });
    return data;
  };
};

const store = createPinia();
store.use(
  piniaPlugin({
    key: 'pinia',
  }),
);

export default store;
