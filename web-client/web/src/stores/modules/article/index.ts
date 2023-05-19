import { defineStore } from 'pinia';
import { Names } from '../../types/store-name';
import type { QueryArticleVO } from '@/api/types/article';

const useArticleDetailStore = defineStore(Names.ARTICLE, {
  state: () => {
    return {
      articleDetail: <QueryArticleVO>{},
    };
  },
  getters: {
    getArticleDetail(): QueryArticleVO {
      return this.articleDetail;
    },
  },
  actions: {
    setArticleDetail(articleDetail: QueryArticleVO) {
      return new Promise<QueryArticleVO>(resolve => {
        this.articleDetail = articleDetail;
        resolve(articleDetail);
      });
    },
  },
});
export { useArticleDetailStore };
