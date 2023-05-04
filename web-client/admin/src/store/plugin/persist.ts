import type { PiniaPluginContext } from 'pinia';
import { toRaw } from 'vue';

interface persistType<S, Store> {
  enable: boolean;
  option: Partial<{
    key: string;
    storageType: 'localStorage' | 'sessionStorage';
    include: (keyof S)[];
    exclude: (keyof S)[];
  }>;
  restoreState?: ((store: Store) => void) | boolean;
}

declare module 'pinia' {
  export interface DefineStoreOptionsBase<S, Store> {
    persist?: Partial<persistType<S, Store>>;
  }
}

export default ({ options, store }: PiniaPluginContext) => {
  const { persist } = options;
  if (!persist?.enable || typeof persist !== 'object') {
    return;
  }
  !persist.option && (persist.option = {});
  const key = (persist.option!.key = persist.option?.key || store.$id);
  const storageType = (persist.option!.storageType = persist.option?.storageType || 'localStorage');

  // 恢复state缓存
  if (persist.restoreState) {
    if (typeof persist.restoreState === 'function') {
      persist.restoreState.call(persist, store);
    } else {
      const storage = window[storageType];
      const data = storage.getItem(persist.option!.key);
      store.$patch(data ? JSON.parse(data) : {});
    }
  }

  // 持久化缓存
  store.$subscribe(
    () => {
      const persistValue = JSON.parse(JSON.stringify(toRaw(store.$state)));
      if (persist.option?.include || persist.option?.exclude) {
        Object.keys(persistValue).forEach(key => {
          if (
            (persist.option?.include && !persist.option?.include?.includes(key)) ||
            (persist.option?.exclude && persist.option?.exclude?.includes(key))
          ) {
            persistValue[key] = undefined;
          }
        });
      }
      window[storageType].setItem(key, JSON.stringify(toRaw(store.$state)));
    },
    { detached: true },
  );
};
