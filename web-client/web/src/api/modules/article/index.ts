import http from '@/api';
import { Method, type Res, type PageRes } from '@/api/types/common';
import type { QueryArticleVO, QueryArticleDTO, QueryUserArticleVO, QueryArticleInfoVO } from '@/api/types/article';

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

  // 根据用户id查询对应的文章数量和用户头像
  public queryUserArticleCountAndAvatar = (userId: string): Promise<Res<QueryUserArticleVO>> => {
    return http.request({
      url: `/articles/info/${userId}`,
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
}

const article = new Article();
export default article;
