import { createPinia } from 'pinia';
import persist from './plugin/persist';

// interface Options {
//   prefix: string;
//   expire: number;
// }

// interface Storage {
//   // eslint-disable-next-line @typescript-eslint/no-explicit-any
//   data: any;
//   expire: number;
// }

// const setStorage = (key: string, value: Storage) => {
//   localStorage.setItem(key, JSON.stringify(value));
// };

// const getStorage = (key: string) => {
//   const value = JSON.parse(localStorage.getItem(key) ?? '{}');
//   if (value) {
//     if (value.expire < Date.now()) {
//       console.log('storage已过期, removed');
//       localStorage.removeItem(key);
//       return {};
//     }
//   }
//   return value;
// };

// const piniaPlugin = (options: Options) => {
//   return (context: PiniaPluginContext) => {
//     const { store } = context;
//     store.$subscribe(() => {
//       setStorage(`${options.prefix}-${store.$id}`, <Storage>{
//         data: toRaw(store.$state),
//         expire: options.expire,
//       });
//     });
//     return getStorage(`${options.prefix}-${store.$id}`);
//   };
// };

const store = createPinia();
// store.use(
//   piniaPlugin({
//     prefix: 'pinia',
//     expire: Date.now() + 1000 * 60 * 60 * 24 * 7,
//   }),
// );
store.use(persist);

export default store;
