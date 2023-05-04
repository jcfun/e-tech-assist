<template>
  <TableBody>
    <template #header>
      <TableHeader ref="tableHeaderRef" :show-filter="false" title="角色搜索" @search="onSearch" @reset-search="onResetSearch">
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
                  <a-select v-model="item.value.value" style="width: 193.5px" :placeholder="item.placeholder" allow-clear>
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
            <template v-else-if="item.key === 'disableFlag'" #cell="{ record }">
              <a-tag color="blue" v-if="record.disableFlag === '0'">启用</a-tag>
              <a-tag color="red" v-else>禁用</a-tag>
            </template>
            <template v-else-if="item.key === 'actions'" #cell="{ record }">
              <a-space>
                <a-button type="text" size="medium" @click="onUpdateItem(record)">编辑</a-button>
                <a-button type="text" size="medium" @click="onUpdateStatus(record)">{{ record.disableFlag == '0' ? '禁用' : '启用' }}</a-button>
                <a-button type="text" size="medium" @click="onDeleteItem(record)">删除</a-button>
              </a-space>
              <a-button type="text" size="medium" @click="onShowMenu(record)"> 角色权限 </a-button>
            </template>
          </a-table-column>
        </template>
      </a-table>
    </template>
    <template #footer>
      <TableFooter ref="tableFooterRef" :pagination="pagination" />
    </template>
  </TableBody>
  <ModalDialog ref="modalDialogRef" :title="actionTitle" @confirm="onConfirm">
    <template #content>
      <a-form :model="roleDTO">
        <a-form-item
          v-for="item of formItems"
          :class="[item.required ? 'form-item__require' : 'form-item__no_require']"
          :label="item.label"
          :key="item.key"
        >
          <template v-if="item.type === 'textarea'">
            <a-textarea v-model="item.value.value" :placeholder="item.placeholder" :auto-size="{ minRows: 3, maxRows: 5 }" />
          </template>
          <template v-if="item.type === 'input'">
            <a-input :placeholder="item.placeholder" v-model="item.value.value">
              <template #prepend v-if="item.key === 'code'">
                {{ ROLE_CODE_FLAG }}
              </template>
            </a-input>
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
  <ModalDialog ref="menuModalDialogRef" title="编辑角色权限" @confirm="onConfirmPerm">
    <template #content>
      <a-tree :data="permTree" show-line checkable v-model:expanded-keys="defaultExpandedKeys" v-model:checked-keys="defaultCheckedKeys" />
    </template>
  </ModalDialog>
</template>

<script setup lang="ts">
  import { usePagination, useRowKey, useTable, useTableColumn, useTableHeight } from '@/hooks/table';
  import { Message, Modal } from '@arco-design/web-vue';
  import { getCurrentInstance, onMounted, ref } from 'vue';
  import type { CreateRoleDTO, UpdateRoleDTO, QueryRoleDTO, RoleVO, UpdateRolePermDTO } from '@/api/types/role';
  import type { FormItem, ModalDialogType } from '@/components/types';
  import role from '@/api/requests/role';
  import perm from '@/api/requests/perm';
  import type { PermVO } from '@/api/types/perm';

  const table = useTable();
  const rowKey = useRowKey('id');

  // 变量区开始-----------------------------------
  const ROLE_CODE_FLAG = 'ROLE_';
  // 弹出框标题
  const actionTitle = ref('添加角色');
  // dto
  let roleDTO: any = {};
  const createRoleDTO = ref({} as CreateRoleDTO);
  const updateRoleDTO = ref({} as UpdateRoleDTO);
  const queryRoleDTO = ref({
    pageNo: 1,
    pageSize: 10,
  } as QueryRoleDTO);
  const updateRolePermDTO = ref({} as UpdateRolePermDTO);
  // 权限树
  const permTree = ref([] as Array<any>);
  const defaultCheckedKeys = ref([] as Array<string>);
  const defaultExpandedKeys = ref([] as Array<string>);
  // 新增更新弹出框vnode
  const modalDialogRef = ref<ModalDialogType | null>(null);
  // 角色权限弹出框vnode
  const menuModalDialogRef = ref<ModalDialogType | null>(null);
  // 添加还是修改
  let actionType = ref('add');
  // 表格项
  const tableColumns = useTableColumn([
    table.indexColumn,
    {
      title: '角色名称',
      key: 'name',
      dataIndex: 'name',
    },
    {
      title: '角色编号',
      key: 'code',
      dataIndex: 'code',
    },
    {
      title: '状态',
      key: 'disableFlag',
      dataIndex: 'disableFlag',
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
      title: '描述',
      key: 'description',
      dataIndex: 'description',
    },
    {
      title: '操作',
      key: 'actions',
      dataIndex: 'actions',
    },
  ]);
  // 修改表单项
  const formItems: any[] = [
    {
      label: '角色名称',
      type: 'input',
      key: 'name',
      value: ref(''),
      required: true,
      placeholder: '请输入角色名称',
      validator: function () {
        if (!this.value.value) {
          Message.error(this.placeholder || '');
          return false;
        }
        return true;
      },
    },
    {
      label: '角色编号',
      key: 'code',
      value: ref(''),
      type: 'input',
      required: true,
      placeholder: '请输入角色编号',
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
      placeholder: '请选择是否启用角色',
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
      label: '用户描述',
      key: 'description',
      value: ref(''),
      type: 'textarea',
      placeholder: '请输入用户描述',
    },
  ];
  // 搜索表单项
  const searchItems: Array<FormItem> = [
    {
      key: 'name',
      label: '角色名称',
      type: 'input',
      placeholder: '请输入角色名称',
      value: ref(''),
      reset: function () {
        this.value.value = '';
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
    {
      key: 'disableFlag',
      label: '是否启用',
      type: 'select',
      placeholder: '请选择是否启用',
      value: ref(''),
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
        this.value.value = '';
      },
    },
  ];
  // 变量区结束-----------------------------------

  // 方法区开始-----------------------------------
  // 刷新按钮
  const doRefresh = () => {
    queryRoleDTO.value.pageNo = pagination.page;
    queryRoleDTO.value.pageSize = pagination.pageSize;
    getRoles(queryRoleDTO.value);
  };
  const pagination = usePagination(doRefresh);
  // 获取用户信息
  const getRoles = (data: QueryRoleDTO) => {
    role.getRolesFq(data).then(res => {
      table.handleSuccess(res?.data?.data);
      pagination.setTotalSize(res?.data?.total);
    });
  };
  // 搜索
  const onSearch = () => {
    const tempDTO: { [key: string]: string | number | string[] | undefined } = {};
    searchItems.forEach((it: any) => {
      tempDTO[it.key] = it.value.value;
    });
    queryRoleDTO.value = {
      ...tempDTO,
      createTimeStart: tempDTO.createTime ? (tempDTO.createTime as string[])[0] : undefined,
      createTimeEnd: tempDTO.createTime ? (tempDTO.createTime as string[])[1] : undefined,
    } as QueryRoleDTO;
    getRoles(queryRoleDTO.value);
  };
  // 重置搜索框
  const onResetSearch = () => {
    searchItems.forEach((it: any) => {
      it.reset ? it.reset() : (it.value.value = '');
    });
  };
  // 新增按钮
  const onAddItem = () => {
    actionTitle.value = '添加用户';
    modalDialogRef.value?.toggle();
    // 设置action为新增
    actionType.value = 'createRole';
    roleDTO = createRoleDTO;
  };
  // 更新按钮
  const onUpdateItem = (item: any) => {
    modalDialogRef.value?.toggle();
    actionTitle.value = '编辑用户';
    formItems.forEach((it: any) => {
      const propName = item[it.key] ? item[it.key] : '';
      it.value.value = propName;
    });
    updateRoleDTO.value.id = item.id;
    roleDTO = updateRoleDTO;
    // 设置action为更新
    actionType.value = 'updateRole';
  };
  // 确认按钮
  const onConfirm = () => {
    if (formItems.every((it: any) => (it.validator ? it.validator() : true))) {
      // 校验通过后为DTO赋值
      formItems.forEach((item: any) => {
        if (item.key == 'code') {
          roleDTO.value.code = item.value.value.startsWith(ROLE_CODE_FLAG) ? item.value.value : ROLE_CODE_FLAG + item.value.value;
        } else if (item.value.value != '' && item.value.value != undefined && item.value.value != null) {
          roleDTO.value[item.key] = item.value.value;
        }
      });
      modalDialogRef.value?.toggle();
      role[actionType.value as keyof typeof role](roleDTO.value as any).then((res: any) => {
        if (res.code == 200) {
          Message.success(res.msg);
          formItems.forEach((it: any) => {
            if (it.reset) {
              it.reset();
            } else if (it.type != 'radio') {
              it.value.value = '';
            }
          });
          doRefresh();
        } else {
          Message.error(res.msg);
        }
      });
    }
  };
  // 角色权限按钮
  const onShowMenu = (item: RoleVO) => {
    perm
      .getPerms()
      .then(res => {
        permTree.value = [];
        defaultExpandedKeys.value = [];
        permTree.value = handlePermData(res.data.data, defaultExpandedKeys.value);
        menuModalDialogRef.value?.toggle();
      })
      .catch(console.log);
    defaultCheckedKeys.value = [];
    defaultCheckedKeys.value = item.perms.map((it: any) => it.id);
    updateRolePermDTO.value.id = item.id;
  };
  // 构建权限数据
  const handlePermData = (permData: Array<PermVO>, defaultExpandedKeys: Array<string>) => {
    const tempPerms = [] as Array<any>;
    permData.forEach(it => {
      const tempPerm = {} as any;
      tempPerm.key = it.id;
      tempPerm.title = it.name;
      if (it.children) {
        defaultExpandedKeys.push(tempPerm.key as string);
        tempPerm.children = handlePermData(it.children, defaultExpandedKeys);
      }
      tempPerms.push(tempPerm);
    });
    return tempPerms;
  };
  // 确认修改角色权限
  const onConfirmPerm = () => {
    updateRolePermDTO.value.permIds = defaultCheckedKeys.value;
    role.updateRolePerm(updateRolePermDTO.value).then(res => {
      if (res.code == 200) {
        Message.success(res.msg);
        doRefresh();
      } else {
        Message.error(res.msg);
      }
    });
    menuModalDialogRef.value?.toggle();
  };
  // 删除
  const onDeleteItem = (item: any) => {
    Modal.confirm({
      title: '提示',
      content: '确定要删除此数据吗？',
      cancelText: '取消',
      okText: '删除',
      onOk: () => {
        role.deleteRole(item.id).then(res => {
          if (res.code == 200) {
            Message.success(res.msg);
            doRefresh();
          } else {
            Message.error(res.msg);
          }
        });
      },
    });
  };
  // 启用禁用
  const onUpdateStatus = (item: any) => {
    let id = item.id;
    let disableFlag = item?.disableFlag == '0' ? '1' : '0';
    role.updateRoleStatus(id, disableFlag).then(res => {
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
