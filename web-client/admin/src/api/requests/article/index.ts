import http from '@/api';
import type { QueryArticleDTO, QueryArticleVO } from '@/api/types/article';
import { Method, type PageRes, type Res } from '@/api/types/common';
class Article {
  // 多条件模糊查询
  public getArticlesFq = (data: QueryArticleDTO): Promise<Res<PageRes<Array<QueryArticleVO>>>> => {
    return http.request({
      url: '/articles/fq',
      method: Method.POST,
      data,
    });
  };

  public deleteArticle = (id: string): Promise<Res<number>> => {
    return http.request({
      url: `/article/${id}`,
      method: Method.DELETE,
    });
  };

  public updateTopArticle = (id: string, flag = '0'): Promise<Res<number>> => {
    return http.request({
      url: `/article/top/${id}/${flag}`,
      method: Method.PATCH,
    });
  };

  public updateArticleStatus = (id: string, status = '0'): Promise<Res<number>> => {
    return http.request({
      url: `/article/status/${id}/${status}`,
      method: Method.PATCH,
    });
  };

  // 根据用户id查询对应的文章数量和用户头像
  public queryUserArticleCount = (userId: string): Promise<Res<number>> => {
    return http.request({
      url: `/articles/count/${userId}`,
      method: Method.GET,
    });
  };
}

const article = new Article();
export default article;
