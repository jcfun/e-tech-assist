interface CreateArticleDTO {
  title: string;
  content: string;
  cover: string;
  categoryId: string;
  tagIds: string;
  status?: string;
}

interface UpdateArticleDTO {
  id: string;
  title: string;
  cover: string;
  content: string;
  status: string;
}

export type { CreateArticleDTO, UpdateArticleDTO };
