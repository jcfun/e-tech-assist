<template>
  <TableBody>
    <template #header>
      <TableHeader ref="tableHeaderRef" :show-filter="false" title="消息搜索" @search="onSearch" @reset-search="onResetSearch">
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
                <template v-if="item.type === 'select' && item.key !== 'msgType'">
                  <a-select v-model="item.value.value" :placeholder="item.placeholder" allow-clear>
                    <a-option v-for="optionItem of item.optionItems" :key="optionItem.value" :value="optionItem.value">
                      {{ optionItem.label }}
                    </a-option>
                  </a-select>
                </template>
                <template v-if="item.type === 'select' && item.key === 'msgType'">
                  <a-select v-model="item.value.value" :placeholder="item.placeholder">
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
            :width="item.width"
          >
            <template v-if="item.key === 'index'" #cell="{ rowIndex }">
              {{ rowIndex + 1 }}
            </template>
            <template v-else-if="item.key === 'successFlag'" #cell="{ record }">
              <a-tag :color="record.successFlag === '0' ? 'red' : 'blue'">
                {{ record.successFlag === '0' ? '失败' : '成功' }}
              </a-tag>
            </template>
            <template v-else-if="item.key === 'sendMethod'" #cell="{ record }">
              {{
                record.sendMethod === '0' ? '邮箱、短信、公众号' : record.sendMethod === '1' ? '邮件' : record.sendMethod === '2' ? '短信' : '公众号'
              }}
            </template>
            <template v-else-if="item.key === 'msgType'" #cell="{ record }">
              {{ record.msgType === '0' ? '发送' : '回复' }}
            </template>
            <template v-else-if="item.key === 'disableFlag'" #cell="{ record }">
              <a-tag color="blue" v-if="record.disableFlag === '0'">启用</a-tag>
              <a-tag color="red" v-else>禁用</a-tag>
            </template>
            <template v-else-if="item.key === 'readFlag'" #cell="{ record }">
              {{ record.readFlag === '0' ? '未读' : '已读' }}
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
              <a-popover title="内容">
                <span>{{ `${record.content.replace('\n', '').slice(0, 20)}${record.content.length > 20 ? '...' : ''}` }}</span>
                <template #content>
                  <p>{{ record.content }}</p>
                </template>
              </a-popover>
            </template>
            <template v-else-if="item.key === 'actions'" #cell="{ record }">
              <a-space>
                <a-button type="text" size="medium" @click="onUpdateDisableFlag(record)">{{ record.disableFlag == '0' ? '禁用' : '启用' }}</a-button>
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
  import { Message } from '@arco-design/web-vue';
  import { getCurrentInstance, onMounted, ref } from 'vue';
  import type { QueryQuickMsgDTO } from '@/api/types/quickMsg';
  import type { FormItem } from '@/components/types';
  import quickMsg from '@/api/requests/quick-msg';

  const table = useTable();
  const rowKey = useRowKey('id');
  // 变量区开始-----------------------------------
  const queryQuickMsgDTO = ref({
    pageNo: 1,
    pageSize: 10,
  } as QueryQuickMsgDTO);
  // 表格项
  const tableColumns = useTableColumn([
    table.indexColumn,
    {
      title: '发送',
      key: 'senderEmail',
      dataIndex: 'senderEmail',
    },
    {
      title: '接收',
      key: 'recipientEmail',
      dataIndex: 'recipientEmail',
    },
    {
      title: '标题',
      key: 'title',
      dataIndex: 'title',
    },
    {
      title: '内容',
      key: 'content',
      dataIndex: 'content',
    },
    {
      title: '发送状态',
      key: 'successFlag',
      dataIndex: 'successFlag',
    },
    {
      title: '发送类型',
      key: 'sendMethod',
      dataIndex: 'sendMethod',
    },
    {
      title: '描述',
      key: 'description',
      dataIndex: 'description',
    },
    {
      title: '消息类型',
      key: 'msgType',
      dataIndex: 'msgType',
    },
    {
      title: '消息状态',
      key: 'disableFlag',
      dataIndex: 'disableFlag',
    },
    {
      title: '是否已读',
      key: 'readFlag',
      dataIndex: 'readFlag',
    },
    {
      title: '创建时间',
      key: 'createTime',
      dataIndex: 'createTime',
    },
    {
      title: '创建者',
      key: 'creator',
      dataIndex: 'creator',
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
      key: 'sender',
      label: '发送用户',
      type: 'input',
      placeholder: '请输入账号/手机号/邮箱',
      value: ref(''),
      reset: function () {
        this.value.value = '';
      },
    },
    {
      key: 'recipient',
      label: '接收用户',
      type: 'input',
      placeholder: '请输入账号/手机号/邮箱',
      value: ref(''),
      reset: function () {
        this.value.value = '';
      },
    },
    {
      key: 'title',
      label: '消息标题',
      type: 'input',
      placeholder: '请输入标题',
      value: ref(''),
      reset: function () {
        this.value.value = '';
      },
    },
    {
      key: 'sendMethod',
      label: '发送方式',
      value: ref(),
      type: 'select',
      placeholder: '请选择发送方式',
      optionItems: [
        {
          label: '邮件、短信、公众号',
          value: '0',
        },
        {
          label: '邮件',
          value: '1',
        },
        {
          label: '短信',
          value: '2',
        },
        {
          label: '公众号',
          value: '3',
        },
      ],
      reset: function () {
        this.value.value = undefined;
      },
    },
    {
      key: 'msgType',
      label: '消息类型',
      value: ref('0'),
      type: 'select',
      placeholder: '请选择消息类型',
      optionItems: [
        {
          label: '发送',
          value: '0',
        },
        {
          label: '回复',
          value: '1',
        },
      ],
      reset: function () {
        this.value.value = '0';
      },
    },
    {
      key: 'disableFlag',
      label: '是否启用',
      value: ref(),
      type: 'select',
      placeholder: '请选择是否启用',
      optionItems: [
        {
          label: '启用',
          value: '0',
        },
        {
          label: '禁用',
          value: '1',
        },
      ],
      reset: function () {
        this.value.value = undefined;
      },
    },
    {
      key: 'readFlag',
      label: '是否已读',
      value: ref(),
      type: 'select',
      placeholder: '请选择是否已读',
      optionItems: [
        {
          label: '未读',
          value: '0',
        },
        {
          label: '已读',
          value: '1',
        },
      ],
      reset: function () {
        this.value.value = undefined;
      },
    },
    {
      key: 'successFlag',
      label: '是否成功',
      value: ref(),
      type: 'select',
      placeholder: '请选择是否成功',
      optionItems: [
        {
          label: '成功',
          value: '1',
        },
        {
          label: '失败',
          value: '0',
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
      value: ref<string[]>(),
      reset: function () {
        this.value.value = [];
      },
    },
  ];
  // 变量区结束-----------------------------------

  // 方法区开始-----------------------------------
  // 刷新按钮
  const doRefresh = () => {
    queryQuickMsgDTO.value.pageNo = pagination.page;
    queryQuickMsgDTO.value.pageSize = pagination.pageSize;
    getQuickMsgs(queryQuickMsgDTO.value);
  };
  const pagination = usePagination(doRefresh);
  // 获取用户信息
  const getQuickMsgs = (data: QueryQuickMsgDTO) => {
    table.tableLoading.value = true;
    quickMsg.getQuickMsgsFq(data).then(res => {
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
    queryQuickMsgDTO.value = {
      ...tempDTO,
      createTimeStart: tempDTO.createTime ? (tempDTO.createTime as string[])[0] : undefined,
      createTimeEnd: tempDTO.createTime ? (tempDTO.createTime as string[])[1] : undefined,
    } as QueryQuickMsgDTO;
    getQuickMsgs(queryQuickMsgDTO.value);
  };
  // 重置搜索框
  const onResetSearch = () => {
    searchItems.forEach((it: any) => {
      it.reset ? it.reset() : (it.value.value = '');
    });
  };
  // 启用禁用
  const onUpdateDisableFlag = (item: any) => {
    let id = item.id;
    let disableFlag = item?.disableFlag == '0' ? '1' : '0';
    quickMsg.updateDisableFlag(id, disableFlag).then(res => {
      if (res.code == 200) {
        Message.success(res.msg);
        doRefresh();
      } else {
        Message.error(res.msg);
      }
    });
  };

  // 生命周期函数
  onMounted(async () => {
    table.tableHeight.value = await useTableHeight(getCurrentInstance());
    doRefresh();
  });
  // 方法区结束-----------------------------------
</script>
