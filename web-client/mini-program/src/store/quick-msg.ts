import { defineStore } from 'pinia';
import { Names } from './store-name';
import type { QueryQuickMsgVO } from '@/api/types/quickMsg';

export const useQuickMsgDetailStore = defineStore(Names.QUICK_MSG, {
  state: () => {
    return {
      quickMsgDetail: <QueryQuickMsgVO>{},
    };
  },
  getters: {
    getQuickMsgDetail(): QueryQuickMsgVO {
      return this.quickMsgDetail;
    },
  },
  actions: {
    setQuickMsgDetail(quickMsgDetail: QueryQuickMsgVO) {
      this.quickMsgDetail = quickMsgDetail;
    },
  },
});
