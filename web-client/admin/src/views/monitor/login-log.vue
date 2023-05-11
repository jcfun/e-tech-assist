<template>
  <TableBody>
    <template #header>
      <TableHeader ref="tableHeaderRef" :show-filter="false" title="日志搜索" @search="onSearch" @reset-search="onResetSearch">
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
            :width="item.width"
          >
            <template v-if="item.key === 'index'" #cell="{ rowIndex }">
              {{ rowIndex + 1 }}
            </template>
            <template v-else-if="item.key === 'successFlag'" #cell="{ record }">
              <a-tag color="red" v-if="record.successFlag === '0'">失败</a-tag>
              <a-tag color="blue" v-else>成功</a-tag>
            </template>
            <template v-else-if="item.key === 'userAgent'" #cell="{ record }">
              {{
                record?.userAgent.includes('Chrome')
                  ? record?.userAgent.slice(record.userAgent.indexOf('Chrome'), record.userAgent.indexOf('Chrome') + 10)
                  : record?.userAgent.includes('Firefox')
                  ? record?.userAgent.slice(record.userAgent.indexOf('Firefox'), record.userAgent.indexOf('Firefox') + 11)
                  : '其它'
              }}
            </template>
            <template v-else-if="item.key === 'method'" #cell="{ record }">
              {{ record.method === '0' ? '微信小程序' : record.method === '1' ? 'web前台' : 'web后台' }}
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
  import { getCurrentInstance, onMounted, ref } from 'vue';
  import type { QueryLoginLogDTO } from '@/api/types/login';
  import type { FormItem } from '@/components/types';
  import login from '@/api/requests/login';

  const table = useTable();
  const rowKey = useRowKey('id');
  // 变量区开始-----------------------------------
  const queryLoginLogDTO = ref({
    pageNo: 1,
    pageSize: 10,
  } as QueryLoginLogDTO);
  // 表格项
  const tableColumns = useTableColumn([
    table.indexColumn,
    {
      title: '账号',
      key: 'account',
      dataIndex: 'account',
    },
    {
      title: '登录IP',
      key: 'ip',
      dataIndex: 'ip',
    },
    {
      title: '登录地址',
      key: 'location',
      dataIndex: 'location',
    },
    {
      title: '设备Mac',
      key: 'mac',
      dataIndex: 'mac',
    },
    {
      title: '客户端',
      key: 'userAgent',
      dataIndex: 'userAgent',
    },
    {
      title: '登录方式',
      key: 'method',
      dataIndex: 'method',
    },
    {
      title: '登录状态',
      key: 'successFlag',
      dataIndex: 'successFlag',
    },
    {
      title: '描述',
      key: 'description',
      dataIndex: 'description',
    },
    {
      title: '访问时间',
      key: 'createTime',
      dataIndex: 'createTime',
    },
  ]);
  // 搜索表单项
  const searchItems: Array<FormItem> = [
    {
      key: 'identity',
      label: '用户标识',
      type: 'input',
      placeholder: '请输入账号/手机号/邮箱',
      value: ref(''),
      reset: function () {
        this.value.value = '';
      },
    },
    {
      key: 'method',
      label: '登录方式',
      value: ref(),
      type: 'select',
      placeholder: '请选择登录方式',
      optionItems: [
        {
          label: '微信小程序',
          value: '0',
        },
        {
          label: 'web前台',
          value: '1',
        },
        {
          label: 'web后台',
          value: '2',
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
          label: '失败',
          value: '0',
        },
        {
          label: '成功',
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
    queryLoginLogDTO.value.pageNo = pagination.page;
    queryLoginLogDTO.value.pageSize = pagination.pageSize;
    getLoginLogsFq(queryLoginLogDTO.value);
  };
  const pagination = usePagination(doRefresh);
  // 获取登录日志信息
  const getLoginLogsFq = (data: QueryLoginLogDTO) => {
    table.tableLoading.value = true;
    login.getLoginLogsFq(data).then(res => {
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
    queryLoginLogDTO.value = {
      ...tempDTO,
      createTimeStart: tempDTO.createTime ? (tempDTO.createTime as string[])[0] : undefined,
      createTimeEnd: tempDTO.createTime ? (tempDTO.createTime as string[])[1] : undefined,
    } as QueryLoginLogDTO;
    getLoginLogsFq(queryLoginLogDTO.value);
  };
  // 重置搜索框
  const onResetSearch = () => {
    searchItems.forEach((it: any) => {
      it.reset ? it.reset() : (it.value.value = '');
    });
  };

  // 生命周期函数
  onMounted(async () => {
    table.tableHeight.value = await useTableHeight(getCurrentInstance());
    doRefresh();
  });
  // 方法区结束-----------------------------------
</script>
