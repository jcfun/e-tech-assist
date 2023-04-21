import type { TinyEmitter } from 'tiny-emitter';
import { emitKey } from '@/layout/index';
import { inject } from 'vue';
export default function useEmit() {
  return inject<TinyEmitter>(emitKey);
}
