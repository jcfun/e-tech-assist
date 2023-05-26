<template>
  <TableBody>
    <template #header>
      <TableHeader ref="tableHeaderRef" :show-filter="false" title="用户搜索" @search="onSearch" @reset-search="onResetSearch">
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
        <template #table-config>
          <AddButton @add="onAddItem" />
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
            <template v-else-if="item.key === 'hiddenFlag'" #cell="{ record }">
              <a-tag :color="record.hiddenFlag === '0' ? 'blue' : 'red'">
                {{ record.hiddenFlag === '0' ? '否' : '是' }}
              </a-tag>
            </template>
            <template v-else-if="item.key === 'permType'" #cell="{ record }">
              {{ record.permType == '0' ? '目录' : record.permType == '1' ? '菜单' : '接口' }}
            </template>
            <template v-else-if="item.key === 'disableFlag'" #cell="{ record }">
              <a-tag color="blue" v-if="record.disableFlag === '0'">启用</a-tag>
              <a-tag color="red" v-else>禁用</a-tag>
            </template>
            <template v-else-if="item.key === 'icon'" #cell="{ record }">
              <component :is="record.resource || 'IconMenu'" style="font-size: 18px" />
            </template>
            <template v-else-if="item.key === 'actions'" #cell="{ record }">
              <a-space>
                <a-button type="text" size="medium" @click="onUpdateItem(record)">编辑</a-button>
                <a-button type="text" size="medium" :status="record.disableFlag == '1' ? 'normal' : 'danger'" @click="onUpdateDisableFlag(record)">{{
                  record.disableFlag == '0' ? '禁用' : '启用'
                }}</a-button>
                <a-button type="text" status="danger" size="medium" @click="onDeleteItem(record)">删除</a-button>
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
  <ModalDialog ref="modalDialogRef" :title="actionTitle" @confirm="onConfirm" draggable>
    <template #content>
      <a-form :model="permDTO">
        <a-form-item
          :class="[item.required ? 'form-item__require' : 'form-item__no_require']"
          :label="item.label"
          v-for="item of formItems"
          :key="item.key"
        >
          <template v-if="item.type === 'tree-select'">
            <a-tree-select
              v-model="item.value.value"
              style="width: 100%"
              :dropdown-style="{ maxHeight: '400px', overflow: 'auto' }"
              :placeholder="item.placeholder"
              allow-clear
              :data="treeData"
            />
          </template>
          <template v-if="item.type === 'input'">
            <a-input :placeholder="item.placeholder" v-model="item.value.value">
              <template #prepend v-if="item.key === 'code'">
                {{ PERM_CODE_FLAG }}
              </template>
            </a-input>
          </template>
          <template v-if="item.type === 'icon'">
            <IconSelector v-model:value="item.value.value" />
          </template>
          <template v-if="item.type === 'textarea'">
            <a-textarea v-model="item.value.value" :placeholder="item.placeholder" :auto-size="{ minRows: 3, maxRows: 5 }" />
          </template>
          <template v-if="item.type === 'radio'">
            <a-radio-group v-model="item.value.value">
              <a-radio v-for="opt of (item.optionItems as any[])" :value="opt.value" :key="opt.value">
                {{ opt.label }}
              </a-radio>
            </a-radio-group>
          </template>
        </a-form-item>
      </a-form>
    </template>
  </ModalDialog>
</template>

<script setup lang="ts">
  import { usePagination, useRowKey, useTable, useTableColumn, useTableHeight } from '@/hooks/table';
  import { Message, Modal } from '@arco-design/web-vue';
  import { getCurrentInstance, onMounted, ref } from 'vue';
  import type { CreatePermDTO, UpdatePermDTO, QueryPermDTO, PermVO } from '@/api/types/perm';
  import type { FormItem, ModalDialogType } from '@/components/types';
  import perm from '@/api/requests/perm';

  const table = useTable();
  const rowKey = useRowKey('id');
  interface TreeItem {
    title: string;
    key: string;
    children?: TreeItem[];
  }

  // 变量区开始-----------------------------------
  const PERM_CODE_FLAG = 'PERM_';
  // 弹出框标题
  const actionTitle = ref('添加用户');
  // dto
  let permDTO: any = {};
  const createPermDTO = ref({} as CreatePermDTO);
  const updatePermDTO = ref({} as UpdatePermDTO);
  const queryPermDTO = ref({
    pageNo: 1,
    pageSize: 10,
  } as QueryPermDTO);
  // 弹出框vnode
  const modalDialogRef = ref<ModalDialogType | null>(null);
  // 添加还是修改
  let actionType = ref('add');
  // 权限树
  const treeData = ref<Array<TreeItem>>([]);
  // 表格项
  const tableColumns = useTableColumn([
    table.indexColumn,
    {
      title: 'api路径',
      key: 'apiPath',
      dataIndex: 'apiPath',
    },
    {
      title: '权限名称',
      key: 'name',
      dataIndex: 'name',
    },
    {
      title: '权限类型',
      key: 'permType',
      dataIndex: 'permType',
    },
    {
      title: '权限图标',
      key: 'icon',
      dataIndex: 'resource',
    },
    {
      title: '权限名称',
      key: 'name',
      dataIndex: 'name',
    },
    {
      title: '路由编号',
      key: 'code',
      dataIndex: 'code',
    },
    {
      title: '前端路由',
      key: 'route',
      dataIndex: 'route',
    },
    {
      title: '路由名称',
      key: 'routeName',
      dataIndex: 'routeName',
    },
    {
      title: '状态',
      key: 'disableFlag',
      dataIndex: 'disableFlag',
    },
    {
      title: '是否隐藏',
      key: 'hiddenFlag',
      dataIndex: 'hiddenFlag',
    },
    {
      title: '父路由',
      key: 'parentRoute',
      dataIndex: 'parentRoute',
    },
    {
      title: '描述',
      key: 'description',
      dataIndex: 'description',
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
  // 新增/更新表单项
  const formItems: any[] = [
    {
      label: '权限名称',
      type: 'input',
      key: 'name',
      value: ref(''),
      required: true,
      placeholder: '请输入权限名称',
      validator: function () {
        if (!this.value.value) {
          Message.error(this.placeholder || '');
          return false;
        }
        return true;
      },
    },
    {
      label: '父级权限',
      key: 'parentPerm',
      value: ref(''),
      placeholder: '请选择父级权限',
      type: 'tree-select',
      reset: function () {
        this.value.value = '';
      },
    },
    {
      label: '权限类型',
      key: 'permType',
      value: ref('0'),
      type: 'radio',
      placeholder: '请输入权限类型',
      optionItems: [
        {
          label: '目录',
          value: '0',
        },
        {
          label: '菜单',
          value: '1',
        },
        {
          label: '接口',
          value: '2',
        },
      ],
      validator: function () {
        if (!this.value.value) {
          Message.error(this.placeholder || '');
          return false;
        }
        return true;
      },
    },
    {
      label: '是否启用',
      key: 'disableFlag',
      value: ref('0'),
      type: 'radio',
      placeholder: '请选择是否启用权限',
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
    },
    {
      label: '是否隐藏',
      key: 'hiddenFlag',
      value: ref('0'),
      type: 'radio',
      placeholder: '请选择是否隐藏权限',
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
    },
    {
      label: '权限编号',
      key: 'code',
      value: ref(''),
      type: 'input',
      required: true,
      placeholder: '请输入权限编号',
      validator: function () {
        if (!this.value.value) {
          Message.error(this.placeholder || '');
          return false;
        }
        return true;
      },
    },
    {
      label: '前端路由',
      key: 'route',
      value: ref(''),
      type: 'input',
      required: true,
      placeholder: '请输入前端路由',
      validator: function () {
        if (!this.value.value) {
          Message.error(this.placeholder || '');
          return false;
        }
        return true;
      },
    },
    {
      label: '路由名称',
      key: 'routeName',
      value: ref(''),
      type: 'input',
      required: true,
      placeholder: '请输入路由名称',
      validator: function () {
        if (!this.value.value) {
          Message.error(this.placeholder || '');
          return false;
        }
        return true;
      },
    },
    {
      label: '接口路径',
      key: 'apiPath',
      value: ref(''),
      type: 'input',
      placeholder: '请输入接口路径',
    },
    {
      label: '权限图标',
      key: 'resource',
      type: 'icon',
      value: ref(''),
      reset: function () {
        this.value.value = '';
      },
    },
    {
      label: '权限描述',
      key: 'description',
      value: ref(''),
      type: 'textarea',
      placeholder: '请输入权限描述',
    },
  ];
  // 搜索表单项
  const searchItems: Array<FormItem> = [
    {
      key: 'name',
      label: '权限名称',
      type: 'input',
      placeholder: '请输入权限名称',
      value: ref(''),
      reset: function () {
        this.value.value = '';
      },
    },
    {
      key: 'permType',
      label: '权限类型',
      value: ref(),
      type: 'select',
      placeholder: '请选择权限类型',
      optionItems: [
        {
          label: '目录',
          value: '0',
        },
        {
          label: '菜单',
          value: '1',
        },
        {
          label: '接口',
          value: '2',
        },
      ],
      reset: function () {
        this.value.value = undefined;
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
      key: 'hiddenFlag',
      label: '是否隐藏',
      value: ref(),
      type: 'select',
      placeholder: '请选择是否隐藏',
      optionItems: [
        {
          label: '不隐藏',
          value: '0',
        },
        {
          label: '隐藏',
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
    queryPermDTO.value.pageNo = pagination.page;
    queryPermDTO.value.pageSize = pagination.pageSize;
    getPerms(queryPermDTO.value);
  };
  const pagination = usePagination(doRefresh);
  // 获取用户信息
  const getPerms = (data: QueryPermDTO) => {
    table.tableLoading.value = true;
    perm.getPermsFq(data).then(res => {
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
    queryPermDTO.value = {
      ...tempDTO,
      createTimeStart: tempDTO.createTime ? (tempDTO.createTime as string[])[0] : undefined,
      createTimeEnd: tempDTO.createTime ? (tempDTO.createTime as string[])[1] : undefined,
    } as QueryPermDTO;
    getPerms(queryPermDTO.value);
  };
  // 重置搜索框
  const onResetSearch = () => {
    searchItems.forEach((it: any) => {
      it.reset ? it.reset() : (it.value.value = '');
    });
  };
  // 新增按钮
  const onAddItem = () => {
    actionTitle.value = '添加权限';
    modalDialogRef.value?.toggle();
    treeData.value = [];
    perm.getPerms().then(res => {
      if (res.code == 200) {
        res.data.data.forEach(it => {
          treeData.value.push({
            title: it.name,
            key: `${it.id}:${it.route ? it.route : ''}`,
            children: getPermTree(it.children),
          });
        });
      }
    });
    // 设置action为新增
    actionType.value = 'createPerm';
    permDTO = createPermDTO;
  };
  // 构建权限树
  const getPermTree = (perms: Array<PermVO>) => {
    const treeData: Array<TreeItem> = [];
    perms.forEach(it => {
      treeData.push({
        title: it.name,
        key: `${it.id}:${it.route ? it.route : ''}`,
        children: getPermTree(it.children),
      });
    });
    return treeData;
  };
  // 更新按钮
  const onUpdateItem = (item: any) => {
    modalDialogRef.value?.toggle();
    actionTitle.value = '编辑权限';
    formItems.forEach((it: any) => {
      if (it.key == 'parentPerm') {
        let parentPerm = `${item.parentId ? item.parentId : ''}:${item.parentRoute ? item.parentRoute : ''}`;
        it.value.value = parentPerm.startsWith(':') || parentPerm.endsWith(':') ? '' : parentPerm;
      } else {
        const propName = item[it.key] ? item[it.key] : '';
        it.value.value = propName;
      }
    });
    treeData.value = [];
    perm.getPerms().then(res => {
      if (res.code == 200) {
        res.data.data.forEach(it => {
          treeData.value.push({
            title: it.name,
            key: `${it.id}:${it.route ? it.route : ''}`,
            children: getPermTree(it.children),
          });
        });
      }
    });
    updatePermDTO.value.id = item.id;
    permDTO = updatePermDTO;
    // 设置action为更新
    actionType.value = 'updatePerm';
  };
  // 确认按钮
  const onConfirm = () => {
    if (formItems.every((it: any) => (it.validator ? it.validator() : true))) {
      // 校验通过后为DTO赋值
      formItems.forEach((item: any) => {
        if (item.key == 'parentPerm') {
          if (item.value.value == '') {
            permDTO.value.parentId = null;
            permDTO.value.parentRoute = null;
          } else {
            permDTO.value.parentId = item.value.value.split(':')[0];
            permDTO.value.parentRoute = item.value.value.split(':')[1] == '' ? null : item.value.value.split(':')[1];
          }
        } else if (item.key == 'code') {
          permDTO.value.code = item.value.value.startsWith(PERM_CODE_FLAG) ? item.value.value : PERM_CODE_FLAG + item.value.value;
        } else if (item.value.value != '' && item.value.value != undefined && item.value.value != null) {
          permDTO.value[item.key] = item.value.value;
        }
      });
      modalDialogRef.value?.toggle();
      perm[actionType.value as keyof typeof perm](permDTO.value as any).then((res: any) => {
        if (res.code == 200) {
          Message.info(res.msg);
          formItems.forEach((it: any) => {
            if (it.reset) {
              it.reset();
            } else if (it.type != 'radio') {
              it.value.value = '';
            }
          });
          doRefresh();
        }
      });
    }
  };
  // 删除
  const onDeleteItem = (item: any) => {
    Modal.confirm({
      title: '提示',
      content: '确定要删除此数据吗？',
      cancelText: '取消',
      okText: '删除',
      onOk: () => {
        perm.deletePerm(item.id).then(res => {
          if (res.code == 200) {
            Message.info(res.msg);
            doRefresh();
          }
        });
      },
    });
  };
  // 启用禁用
  const onUpdateDisableFlag = (item: any) => {
    let id = item.id;
    let disableFlag = item?.disableFlag == '0' ? '1' : '0';
    perm.updateDisableFlag(id, disableFlag).then(res => {
      if (res.code == 200) {
        Message.info(res.msg);
        doRefresh();
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
