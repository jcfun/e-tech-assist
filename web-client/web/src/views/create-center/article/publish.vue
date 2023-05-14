<template>
  <div class="editor-box">
    <div class="editor-wrap">
      <Toolbar class="editor-toolbar" :editor="editorRef" :defaultConfig="toolbarConfig" :mode="mode" />
      <div class="editor-title-wrap">
        <input v-model="articleTitle" placeholder="请输入标题" type="text" />
      </div>
      <Editor
        class="editor-content"
        v-model="valueHtml"
        :defaultConfig="editorConfig"
        :mode="mode"
        @onCreated="handleCreated"
        @onChange="handleChange"
      />
      <div class="editor-cover-wrap">
        <div class="editor-cover-title">设置文章封面</div>
        <div class="editor-cover">
          <a-upload :show-file-list="false" :custom-request="onUpload">
            <template #upload-button>
              <div v-if="!isEmpty(cover)">
                <img style="width: 300px; height: 160px" :src="cover" />
              </div>
              <div
                v-else
                style="
                  background-color: var(--color-fill-2);
                  color: var(--color-text-1);
                  border: 1px dashed var(--color-fill-4);
                  height: 160px;
                  width: 300px;
                  border-radius: 5px;
                  display: flex;
                  flex-direction: column;
                  align-items: center;
                  justify-content: center;
                "
              >
                <icon-plus size="20" style="color: #636d7b" />
                <span style="color: #636d7b; margin-top: 10px">点击或拖拽文件到此处上传</span>
              </div>
            </template>
          </a-upload>
        </div>
      </div>
      <div class="editor-btn-wrap">
        <a-button type="primary" @click="onFinish('1')">发布</a-button>
        <div style="width: 20px"></div>
        <a-button type="outline" @click="onFinish('0')">保存草稿</a-button>
      </div>
    </div>
    <a-modal v-model:visible="showModal" title="图片裁剪" @ok="handleUpload" draggable :width="900" :mask-style="{ background: '#000000da' }">
      <div style="display: flex; justify-content: center" v-if="showModal">
        <div style="width: 600px; height: 320px"><img id="cropperImg" :src="cropperImage" class="cropper-img" /></div>
        <div style="display: flex; flex-direction: column; margin-left: 20px">
          <span style="margin-bottom: 30px">预览</span>
          <img style="width: 250px; height: 134px" :src="croppedImage" />
        </div>
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
  import '@wangeditor/editor/dist/css/style.css'; // 引入 css
  import { nextTick, onBeforeUnmount, ref, shallowRef } from 'vue';
  import { Editor, Toolbar } from '@wangeditor/editor-for-vue';
  import type { IEditorConfig } from '@wangeditor/editor';
  import http from '@/api';
  import { useUserStore } from '@/stores/modules/user';
  import { Message, type RequestOption } from '@arco-design/web-vue';
  import type { Res } from '@/api/types/common';
  import type { UploadVO } from '@/api/types/upload';
  import upload from '@/api/modules/upload';
  import { isEmpty } from '@/utils/';
  import Cropper from 'cropperjs';
  import 'cropperjs/dist/cropper.css';
  import type { CreateArticleDTO } from '@/api/types/article';
  import article from '@/api/modules/article';
  const userStore = useUserStore();
  const token = `${userStore?.user?.token.tokenType ?? 'Bearer'} ${userStore?.user?.token?.token}`;
  const showModal = ref(false);
  const croppedImage = ref('');
  const cropperImage = ref('');
  const articleTitle = ref('');
  // --------------------编辑器开始------------------------
  // 初始化 MENU_CONF 属性
  const editorConfig: Partial<IEditorConfig> = {
    placeholder: '请输入内容...',
    MENU_CONF: {},
  };
  type InsertFnType = (url: string, alt: string, href: string) => void;
  editorConfig.MENU_CONF!['uploadImage'] = {
    server: `${http.baseUrl}/upload/image`, // 上传图片地址
    headers: {
      Authorization: token,
    },
    // 自定义插入图片
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    customInsert(res: Res<UploadVO>, insertFn: InsertFnType) {
      let { data } = res;
      insertFn(data.url, data.alt, data.href);
    },
  };
  const mode = ref('default'); // 编辑器模式，可选值为 default 和 simple
  // 编辑器实例，必须用 shallowRef
  const editorRef = shallowRef();
  // 内容 HTML
  const valueHtml = ref('');
  const toolbarConfig = {};
  const handleChange = () => {
    console.log(valueHtml.value);
  };
  // 组件销毁时，也及时销毁编辑器
  onBeforeUnmount(() => {
    const editor = editorRef.value;
    if (editor == null) return;
    editor.destroy();
  });
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const handleCreated = (editor: any) => {
    editorRef.value = editor; // 记录 editor 实例，重要！
  };
  // --------------------编辑器结束------------------------

  // --------------------文章封面开始------------------------
  const cover = ref('');
  // 图片裁剪
  const onUpload = (option: RequestOption) => {
    const { fileItem } = option;
    cropperImage.value = fileItem.url!;
    showModal.value = true;
    nextTick(() => {
      const image = document.getElementById('cropperImg') as HTMLImageElement;
      const cropper = new Cropper(image, {
        preview: '.small',
        autoCrop: false,
        viewMode: 1,
        dragMode: 'none',
        initialAspectRatio: 1,
        aspectRatio: 15 / 8,
        background: true,
        autoCropArea: 0.6,
        zoomOnWheel: true,
        ready() {
          cropper.crop();
        },
        crop() {
          croppedImage.value = cropper.getCroppedCanvas().toDataURL();
        },
      });
    });
  };
  // 裁剪完成确认
  const handleUpload = async () => {
    const blob = await (await fetch(croppedImage.value)).blob();
    const headers = new Headers();
    headers.append('Authorization', token);
    headers.append('Content-Type', 'multipart/form-data');
    let formData = new FormData();
    formData.append('image', blob);
    let res = await upload.uploadImage(formData, headers);
    cover.value = res.data!.url ?? '';
  };
  // --------------------文章封面结束------------------------
  const onFinish = async (status: string) => {
    const dto = {
      title: articleTitle.value,
      content: valueHtml.value,
      cover: cover.value,
      categoryId: '',
      tagIds: '',
      status,
    } as CreateArticleDTO;
    let res = await article.createArticle(dto);
    if (res.code == 200) {
      Message.info(res.msg);
    }
  };
</script>

<style scoped lang="scss">
  .editor-box {
    width: 100%;
    min-width: 500px;
    padding: 30px 0;
    padding-right: 10%;
    background-color: #f9f9f9;
    .editor-toolbar {
      font-size: 30px;
    }
    .editor-wrap {
      min-width: 800px;
      overflow: hidden;
      background-color: #fff;
      box-shadow: 0 0 5px #c9c9c9;
      border-radius: 5px;
      padding: 20px 50px;
      .editor-title-wrap {
        width: 100%;
        border-bottom: 1px solid #e8e8e8;
        padding: 20px 0;
        input {
          width: inherit;
          font-size: 30px;
          border: none;
          outline: none;
          line-height: 1;
        }
      }
      .editor-content {
        height: 600px !important;
        box-sizing: border-box;
        overflow: hidden;
      }
      .editor-cover-wrap {
        border-top: 1px solid #e8e8e8;
        margin-top: 20px;
        .editor-cover-title {
          margin-top: 20px;
        }
        .editor-cover {
          padding: 20px 0;
          width: 280px;
          .arco-upload-wrapper {
            height: 100%;
            .arco-upload-draggable {
              height: 100%;
            }
          }
        }
      }
      .editor-btn-wrap {
        margin-top: 30px;
        display: flex;
        justify-content: center;
      }
    }
  }
  .cropper-img {
    display: block;
    max-width: 100%;
  }
</style>
