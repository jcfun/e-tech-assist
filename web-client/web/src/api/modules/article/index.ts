import http from '@/api';
import { Method, type Res } from '@/api/types/common';
import type { CreateArticleDTO } from '@/api/types/article';

class Article {
  public createArticle = (data: CreateArticleDTO): Promise<Res<null>> => {
    return http.request({
      url: '/article',
      method: Method.POST,
      data,
    });
  };
}

const article = new Article();
export default article;
