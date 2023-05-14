export interface CreateArticleDTO {
  title: string;
  content: string;
  cover: string;
  categoryId: string;
  tagIds: string;
  status?: string;
}
