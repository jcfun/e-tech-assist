import request from '../..';
import { Method, type PageRes, type Res } from '@/api/types/common';
import type { QueryArticleDTO, QueryArticleVO } from '@/api/types/article';

class Article {
  // 多条件模糊查询
  public getArticlesFq = (data: QueryArticleDTO): Promise<Res<PageRes<QueryArticleVO>>> => {
    return request.request({
      url: '/articles/fq',
      method: Method.POST,
      data,
    });
  };
  // 查询热门文章
  public getHotArticles = (): Promise<Res<Array<QueryArticleVO>>> => {
    return request.request({
      url: '/articles/hot',
      method: Method.GET,
    });
  };
  // 更新文章浏览量
  public updateArticleViewCount = (articleId: string): Promise<Res<number>> => {
    return request.request({
      url: `/articles/view/${articleId}`,
      method: Method.PUT,
    });
  };
}
const article = new Article();
export default article;
