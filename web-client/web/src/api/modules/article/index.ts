import http from '@/api';
import { Method, type Res, type PageRes } from '@/api/types/common';
import type { QueryArticleVO, QueryArticleDTO, QueryArticleInfoVO } from '@/api/types/article';

class Article {
  // 多条件模糊查询文章
  public queryArticlesFq = (data: QueryArticleDTO): Promise<Res<PageRes<QueryArticleVO>>> => {
    return http.request({
      url: '/articles/fq',
      method: Method.POST,
      data,
    });
  };

  // 查询置顶文章
  public queryTopArticles = (): Promise<Res<Array<QueryArticleVO>>> => {
    return http.request({
      url: '/articles/top',
      method: Method.GET,
    });
  };

  // 查询热门文章
  public queryHotArticles = (): Promise<Res<Array<QueryArticleVO>>> => {
    return http.request({
      url: '/articles/hot',
      method: Method.GET,
    });
  };

  // 根据用户id查询对应的文章数量
  public queryUserArticleCount = (userId: string): Promise<Res<number>> => {
    return http.request({
      url: `/articles/count/${userId}`,
      method: Method.GET,
    });
  };

  // 根据获取用户文章投稿信息
  public queryArticleInfo = (): Promise<Res<QueryArticleInfoVO>> => {
    return http.request({
      url: `/article/info`,
      method: Method.GET,
    });
  };

  // 更新文章浏览量
  public updateArticleViewCount = (articleId: string): Promise<Res<number>> => {
    return http.request({
      url: `/articles/view/${articleId}`,
      method: Method.PUT,
    });
  };
}

const articles = new Article();
export default articles;
