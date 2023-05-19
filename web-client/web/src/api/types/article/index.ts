interface QueryArticleVO {
  id: string;
  operateTime: string;
  operator: string;
  operatorId: string;
  createTime: string;
  creator: string;
  creatorId: string;
  deleteFlag: string;
  title: string;
  cover: string;
  content: string;
  viewCount: string;
  likeCount: string;
  commentCount: string;
  categoryId: string;
  tagIds: string;
  status: string;
  collectCount: string;
  forwardCount: string;
  topFlag: string;
}

interface QueryArticleDTO {
  createTimeStart?: string;
  createTimeEnd?: string;
  title?: string;
  categoryId?: string;
  tagId?: string;
  status?: string;
  byUserIdFlag?: string;
  topFlag?: string;
  pageNo?: number;
  pageSize?: number;
}

interface QueryUserArticleVO {
  totalArticleCount: number;
  avatar: string;
}

interface QueryArticleInfoVO {
  type: string;
  totalArticleCount: number;
  totalViewCount: number;
  totalLikeCount: number;
  totalCommentCount: number;
  totalCollectCount: number;
  totalForwardCount: number;
}

export type { QueryArticleVO, QueryArticleDTO, QueryUserArticleVO, QueryArticleInfoVO };
