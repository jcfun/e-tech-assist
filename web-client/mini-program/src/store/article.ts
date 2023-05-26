import { defineStore } from 'pinia';
import { Names } from './store-name';
import type { QueryArticleVO } from '@/api/types/article';

export const useArticleDetailStore = defineStore(Names.ARTICLE, {
  state: () => {
    return {
      articleDetail: <QueryArticleVO>{},
    };
  },
  getters: {},
  actions: {
    setQuickMsgDetail(articleDetail: QueryArticleVO) {
      return new Promise<QueryArticleVO>(resolve => {
        this.articleDetail = articleDetail;
        resolve(articleDetail);
      });
    },
  },
});
