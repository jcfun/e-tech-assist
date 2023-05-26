<style lang="scss" scoped>
  .eye-icon-wrap {
    height: 25px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    &:hover {
      background-color: #f2f3f5;
      color: #006fff;
      transition: all 0.3s ease-in;
    }
  }
</style>

<template>
  <TableBody>
    <template #header>
      <TableHeader ref="tableHeaderRef" :show-filter="false" title="文章搜索" @search="onSearch" @reset-search="onResetSearch">
        <template #search-content>
          <a-form layout="inline" :model="{}">
            <a-form-item v-for="item of searchItems" :key="item.key" :label="item.label">
              <template v-if="item.render">
                <FormRender :render="item.render" :formItem="item" />
              </template>
              <template v-else>
                <template v-if="item.type === 'input'">
                  <a-input v-model="item.value.value" :placeholder="item.placeholder" />
                </template>
                <template v-if="item.type === 'select'">
                  <a-select v-model="item.value.value" :placeholder="item.placeholder" allow-clear>
                    <a-option v-for="optionItem of item.optionItems" :key="optionItem.value" :value="optionItem.value">
                      {{ optionItem.label }}
                    </a-option>
                  </a-select>
                </template>
                <template v-if="item.type === 'range-picker'">
                  <a-range-picker showTime v-model="item.value.value" />
                </template>
              </template>
            </a-form-item>
          </a-form>
        </template>
      </TableHeader>
    </template>
    <template #default>
      <a-table :loading="table.tableLoading.value" :bordered="false" :data="table.dataList" :pagination="false" :rowKey="rowKey">
        <template #columns>
          <a-table-column
            v-for="item of tableColumns"
            :key="item.key"
            :align="item.align"
            :title="(item.title as string)"
            :data-index="(item.key as string)"
            :fixed="item.fixed"
            :width="item.key == 'content' ? 70 : item.width"
          >
            <template v-if="item.key === 'index'" #cell="{ rowIndex }">
              {{ rowIndex + 1 }}
            </template>
            <template v-else-if="item.key === 'cover'" #cell="{ record }">
              <a-image width="60" :src="record.cover" />
            </template>
            <template v-else-if="item.key === 'title'" #cell="{ record }">
              <a-popover title="标题">
                <span>{{ `${record.title.replace('\n', '').slice(0, 10)}${record.title.length > 20 ? '...' : ''}` }}</span>
                <template #content>
                  <p>{{ record.title }}</p>
                </template>
              </a-popover>
            </template>
            <template v-else-if="item.key === 'content'" #cell="{ record }">
              <div class="eye-icon-wrap" @click="toDetail(record)">
                <icon-eye size="20" />
              </div>
            </template>
            <template v-else-if="item.key === 'status'" #cell="{ record }">
              <a-tag color="green" v-if="record.status === '0'">草稿</a-tag>
              <a-tag color="red" v-else-if="record.status === '1'">审核中</a-tag>
              <a-tag color="blue" v-else>已发布</a-tag>
            </template>
            <template v-else-if="item.key === 'topFlag'" #cell="{ record }">
              <a-tag color="red" v-if="record.topFlag === '0'">否</a-tag>
              <a-tag color="blue" v-else>是</a-tag>
            </template>
            <template v-else-if="item.key === 'actions'" #cell="{ record }">
              <a-space>
                <a-button type="text" :status="record.status == '2' ? 'danger' : 'normal'" size="medium" @click="onUpdateStatus(record)">{{
                  record.status == '2' ? '重新审核' : '通过审核'
                }}</a-button>
                <a-button type="text" :status="record.topFlag == '0' ? 'normal' : 'danger'" size="medium" @click="onUpdateTop(record)">{{
                  record.topFlag == '0' ? '置顶文章' : '取消置顶'
                }}</a-button>
                <a-button type="text" status="danger" size="medium" @click="onDelete(record)">删除</a-button>
              </a-space>
            </template>
          </a-table-column>
        </template>
      </a-table>
    </template>
    <template #footer>
      <TableFooter ref="tableFooterRef" :pagination="pagination" />
    </template>
  </TableBody>
</template>

<script setup lang="ts">
  import { usePagination, useRowKey, useTable, useTableColumn, useTableHeight } from '@/hooks/table';
  import { Message, Modal } from '@arco-design/web-vue';
  import { getCurrentInstance, onMounted, ref } from 'vue';
  import type { FormItem } from '@/components/types';
  import type { QueryArticleDTO, QueryArticleVO } from '@/api/types/article';
  import article from '@/api/requests/article';
  import { useArticleDetailStore } from '@/store/modules/article';

  const table = useTable();
  const rowKey = useRowKey('id');
  // 变量区开始-----------------------------------
  const queryArticleDTO = ref<QueryArticleDTO>({
    pageNo: 1,
    pageSize: 10,
  });
  // 表格项
  const tableColumns = useTableColumn([
    table.indexColumn,
    {
      title: '标题',
      key: 'title',
      dataIndex: 'title',
    },
    {
      title: '封面',
      key: 'cover',
      dataIndex: 'cover',
    },
    {
      title: '内容',
      key: 'content',
      dataIndex: 'content',
    },
    {
      title: '作者',
      key: 'creator',
      dataIndex: 'creator',
    },
    {
      title: '创建时间',
      key: 'createTime',
      dataIndex: 'createTime',
    },
    {
      title: '最后修改时间',
      key: 'operateTime',
      dataIndex: 'operateTime',
    },
    {
      title: '状态',
      key: 'status',
      dataIndex: 'status',
    },
    {
      title: '置顶',
      key: 'topFlag',
      dataIndex: 'topFlag',
    },
    {
      title: '阅读量',
      key: 'viewCount',
      dataIndex: 'viewCount',
    },
    {
      title: '点赞数',
      key: 'likeCount',
      dataIndex: 'likeCount',
    },
    {
      title: '评论数',
      key: 'commentCount',
      dataIndex: 'commentCount',
    },
    {
      title: '收藏数',
      key: 'collectCount',
      dataIndex: 'collectCount',
    },
    {
      title: '转发数',
      key: 'forwardCount',
      dataIndex: 'forwardCount',
    },
    {
      title: '操作',
      key: 'actions',
      dataIndex: 'actions',
    },
  ]);
  // 搜索表单项
  const searchItems: Array<FormItem> = [
    {
      key: 'title',
      label: '文章标题',
      type: 'input',
      placeholder: '请输入标题',
      value: ref(),
      reset: function () {
        this.value.value = undefined;
      },
    },
    {
      key: 'status',
      label: '文章状态',
      value: ref(),
      type: 'select',
      placeholder: '请选择文章状态',
      optionItems: [
        {
          label: '草稿',
          value: '0',
        },
        {
          label: '审核中',
          value: '1',
        },
        {
          label: '已发布',
          value: '2',
        },
      ],
      reset: function () {
        this.value.value = undefined;
      },
    },
    {
      key: 'topFlag',
      label: '是否置顶',
      value: ref(),
      type: 'select',
      placeholder: '请选择是否置顶',
      optionItems: [
        {
          label: '否',
          value: '0',
        },
        {
          label: '是',
          value: '1',
        },
      ],
      reset: function () {
        this.value.value = undefined;
      },
    },
    {
      key: 'createTime',
      label: '创建日期',
      type: 'range-picker',
      value: ref<Array<string>>(),
      reset: function () {
        this.value.value = undefined;
      },
    },
  ];
  // 变量区结束-----------------------------------

  // 方法区开始-----------------------------------
  // 刷新按钮
  const doRefresh = () => {
    queryArticleDTO.value.pageNo = pagination.page;
    queryArticleDTO.value.pageSize = pagination.pageSize;
    getArticles(queryArticleDTO.value);
  };
  const pagination = usePagination(doRefresh);
  // 获取文章信息
  const getArticles = (data: QueryArticleDTO) => {
    table.tableLoading.value = true;
    article.getArticlesFq(data).then(res => {
      table.handleSuccess(res?.data?.data);
      pagination.setTotalSize(res?.data?.total ? res?.data?.total : 0);
    });
  };
  // 搜索
  const onSearch = () => {
    const tempDTO: { [key: string]: string | number | string[] | undefined } = {};
    searchItems.forEach((it: any) => {
      tempDTO[it.key] = it.value.value;
    });
    const { createTime, ...filteredTempDTO } = tempDTO;
    queryArticleDTO.value = {
      ...queryArticleDTO.value,
      ...filteredTempDTO,
      createTimeStart: (createTime as Array<string>)?.[0] ?? undefined,
      createTimeEnd: (createTime as Array<string>)?.[1] ?? undefined,
    } as QueryArticleDTO;
    getArticles(queryArticleDTO.value);
  };
  // 重置搜索框
  const onResetSearch = () => {
    searchItems.forEach((it: any) => {
      it.reset ? it.reset() : (it.value.value = '');
    });
  };
  // 删除
  const onDelete = (item: any) => {
    Modal.confirm({
      title: '提示',
      content: '确定要删除此数据吗？',
      cancelText: '取消',
      okText: '删除',
      onOk: () => {
        article.deleteArticle(item.id).then(res => {
          if (res.code == 200) {
            Message.info(res.msg);
            doRefresh();
          }
        });
      },
    });
  };
  // 修改文章状态
  const onUpdateStatus = (item: any) => {
    let id = item.id;
    let status = item?.status == '2' ? '1' : '2';
    article.updateArticleStatus(id, status).then(res => {
      if (res.code == 200) {
        Message.info(res.msg);
        doRefresh();
      }
    });
  };
  // 修改文章是否置顶
  const onUpdateTop = (item: any) => {
    let id = item.id;
    let status = item?.topFlag == '0' ? '1' : '0';
    article.updateTopArticle(id, status).then(res => {
      if (res.code == 200) {
        Message.info(res.msg);
        doRefresh();
      }
    });
  };
  const toDetail = (item: QueryArticleVO) => {
    useArticleDetailStore()
      .setArticleDetail(item)
      .then(() => {
        window.open('#/data/article/detail', '_blank');
      });
  };
  // 生命周期函数
  onMounted(async () => {
    table.tableHeight.value = await useTableHeight(getCurrentInstance());
    doRefresh();
  });
  // 方法区结束-----------------------------------
</script>
