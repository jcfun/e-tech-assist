import http from '@/api';
import { Method, type Res } from '@/api/types/common';
import type { UploadVO } from '@/api/types/upload';

class Upload {
  public uploadImage = (data: FormData, headers: Headers): Promise<Res<UploadVO>> => {
    return http.request({
      url: '/upload/image',
      method: Method.POST,
      headers,
      data,
    });
  };
}

const upload = new Upload();
export default upload;
