import http from '@/api';
import { Method, type Res } from '@/api/types/common';
import type { CreateArticleDTO, UpdateArticleDTO } from '@/api/types/create-center/article';

class Article {
  public createArticle = (data: CreateArticleDTO): Promise<Res<number>> => {
    return http.request({
      url: '/article',
      method: Method.POST,
      data,
    });
  };
  public updateArticle = (data: UpdateArticleDTO): Promise<Res<number>> => {
    return http.request({
      url: '/article',
      method: Method.PUT,
      data,
    });
  };
  public deleteArticle = (id: string): Promise<Res<number>> => {
    return http.request({
      url: `/article/${id}`,
      method: Method.DELETE,
    });
  };
}

const article = new Article();
export default article;
