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
                  <a-select v-model="item.value.value" style="width: 227.5px" :placeholder="item.placeholder" allow-clear>
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
            <template v-else-if="item.key === 'gender'" #cell="{ record }">
              <a-tag :color="record.gender === '1' ? 'blue' : record.gender === '2' ? 'red' : 'gray'">
                {{ record.gender === '1' ? '男' : record.gender === '2' ? '女' : '未知' }}
              </a-tag>
            </template>
            <template v-else-if="item.key === 'avatarUrl'" #cell="{ record }">
              <a-avatar v-if="!record.avatarUrl" :size="30" :style="{ backgroundColor: 'var(--color-primary-light-1)' }">
                <IconUser />
              </a-avatar>
              <a-avatar v-else :size="30" trigger-type="mask">
                <img alt="avatar" :src="record.avatarUrl" />
                <template #trigger-icon>
                  <IconEdit />
                </template>
              </a-avatar>
            </template>
            <!-- <template v-else-if="item.key === 'address'" #cell="{ record }">
              {{ `${record.country ? record.country : ''} ${record.province ? record.province : ''} ${record.city ? record.city : ''}` }}
            </template> -->
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
      <a-form :model="userDTO">
        <a-form-item
          :class="[item.required ? 'form-item__require' : 'form-item__no_require']"
          :label="item.label"
          v-for="item of formItems"
          :key="item.key"
        >
          <template v-if="item.type === 'input'">
            <a-input :placeholder="item.placeholder" v-model="item.value.value"> </a-input>
          </template>
          <template v-if="item.type === 'textarea'">
            <a-textarea v-model="item.value.value" :placeholder="item.placeholder" :auto-size="{ minRows: 3, maxRows: 5 }" />
          </template>
          <template v-if="item.type === 'select-multiple'">
            <a-select v-model="item.value.value" multiple :placeholder="item.placeholder">
              <a-option v-for="opt of item.optionItems" :value="opt.value" :key="opt.value">
                {{ opt.label }}
              </a-option>
            </a-select>
          </template>
          <template v-if="item.type === 'radio'">
            <a-radio-group v-model="item.value.value">
              <a-radio v-for="opt of item.optionItems" :value="opt.value" :key="opt.value">
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
  import user from '@/api/requests/user';
  import { usePagination, useRowKey, useTable, useTableColumn, useTableHeight } from '@/hooks/table';
  import { Message, Modal } from '@arco-design/web-vue';
  import { getCurrentInstance, onMounted, ref } from 'vue';
  import type { CreateUserDTO, UpdateUserDTO, QueryUserDTO } from '@/api/types/user';
  import type { FormItem, ModalDialogType, SelectOptionItem } from '@/components/types';
  import role from '@/api/requests/role';

  const table = useTable();
  const rowKey = useRowKey('id');

  // 变量区开始-----------------------------------
  // 弹出框标题
  const actionTitle = ref('添加用户');
  // dto
  let userDTO: any = {};
  const createUserDTO = ref({} as CreateUserDTO);
  const updateUserDTO = ref({} as UpdateUserDTO);
  const queryUserDTO = ref({
    pageNo: 1,
    pageSize: 10,
  } as QueryUserDTO);
  // 弹出框vnode
  const modalDialogRef = ref<ModalDialogType | null>(null);
  // 角色选项
  const optionRoleItems = ref([] as SelectOptionItem[]);
  // 添加还是修改
  let actionType = ref('add');
  // 表格项
  const tableColumns = useTableColumn([
    table.indexColumn,
    {
      title: '账号',
      key: 'account',
      dataIndex: 'account',
    },
    {
      title: '手机号',
      key: 'phoneNumber',
      dataIndex: 'phoneNumber',
    },
    {
      title: '邮箱',
      key: 'email',
      dataIndex: 'email',
    },
    {
      title: '昵称',
      key: 'nickname',
      dataIndex: 'nickname',
    },
    {
      title: '性别',
      key: 'gender',
      dataIndex: 'gender',
    },
    {
      title: '头像',
      key: 'avatarUrl',
      dataIndex: 'avatarUrl',
    },
    {
      title: '状态',
      key: 'disableFlag',
      dataIndex: 'disableFlag',
    },
    // {
    //   title: '地址',
    //   key: 'address',
    //   dataIndex: 'city',
    // },
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
      title: '登录时间',
      key: 'lastLoginTime',
      dataIndex: 'lastLoginTime',
    },
    {
      title: '登录IP',
      key: 'lastLoginIp',
      dataIndex: 'lastLoginIp',
    },
    {
      title: '操作',
      key: 'actions',
      dataIndex: 'actions',
    },
  ]);
  // 新增表单项
  const addFormItem: any = [
    {
      label: '账号',
      type: 'input',
      key: 'account',
      value: ref(''),
      required: true,
      placeholder: '请输入用户账号',
      validator: function () {
        if (!this.value.value) {
          Message.error(this.placeholder || '');
          return false;
        }
        return true;
      },
    },
    {
      label: '昵称',
      key: 'nickname',
      value: ref(''),
      type: 'input',
      required: true,
      placeholder: '请输入用户昵称',
      validator: function () {
        if (!this.value.value) {
          Message.error(this.placeholder || '');
          return false;
        }
        return true;
      },
    },
    {
      label: '手机号',
      key: 'phoneNumber',
      value: ref(''),
      type: 'input',
      required: true,
      placeholder: '请输入用户手机号',
      validator: function () {
        if (!this.value.value) {
          Message.error(this.placeholder || '');
          return false;
        }
        return true;
      },
    },
    {
      label: '密码',
      key: 'password',
      value: ref(''),
      type: 'input',
      placeholder: '请输入用户密码',
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
      label: '角色',
      key: 'roleIds',
      value: ref([]),
      type: 'select-multiple',
      placeholder: '请选择用户角色',
      optionItems: optionRoleItems.value,
      reset: function () {
        this.value.value = [];
      },
    },
    {
      label: '用户描述',
      key: 'description',
      value: ref(''),
      type: 'textarea',
      placeholder: '请输入用户描述',
    },
  ];
  // 更新表单项
  const updateFormItem: any = [
    {
      label: '昵称',
      key: 'nickname',
      value: ref(''),
      type: 'input',
      required: true,
      placeholder: '请输入用户昵称',
      validator: function () {
        if (!this.value.value) {
          Message.error(this.placeholder || '');
          return false;
        }
        return true;
      },
    },
    {
      label: '手机号',
      key: 'phoneNumber',
      value: ref(''),
      type: 'input',
      required: true,
      placeholder: '请输入用户手机号',
      validator: function () {
        if (!this.value.value) {
          Message.error(this.placeholder || '');
          return false;
        }
        return true;
      },
    },
    {
      label: '邮箱',
      key: 'email',
      value: ref(''),
      type: 'input',
      placeholder: '请输入用户邮箱',
    },
    {
      label: '密码',
      key: 'password',
      value: ref(''),
      type: 'input',
      placeholder: '请输入用户密码',
    },
    {
      label: '角色',
      key: 'roleIds',
      value: ref([]),
      type: 'select-multiple',
      placeholder: '请选择用户角色',
      optionItems: optionRoleItems.value,
      reset: function () {
        this.value.value = [];
      },
    },
    {
      label: '用户性别',
      key: 'gender',
      value: ref(''),
      type: 'radio',
      placeholder: '请选择用户性别',
      optionItems: [
        {
          label: '男',
          value: '1',
        },
        {
          label: '女',
          value: '2',
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
  let formItems = addFormItem;
  // 搜索表单项
  const searchItems: Array<FormItem> = [
    {
      key: 'nickname',
      label: '用户昵称',
      type: 'input',
      placeholder: '请输入用户昵称',
      value: ref(''),
      reset: function () {
        this.value.value = '';
      },
    },
    {
      key: 'email',
      label: '用户邮箱',
      value: ref(),
      type: 'input',
      placeholder: '请输入用户邮箱',
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
    {
      key: 'phoneNumber',
      label: '用户手机',
      value: ref(),
      type: 'input',
      placeholder: '请输入用户手机号',
      reset: function () {
        this.value.value = undefined;
      },
    },
    {
      key: 'gender',
      label: '用户姓别',
      value: ref(),
      type: 'select',
      placeholder: '请选择用户姓别',
      optionItems: [
        {
          label: '未知',
          value: '0',
        },
        {
          label: '男',
          value: '1',
        },
        {
          label: '女',
          value: '2',
        },
      ],
      reset: function () {
        this.value.value = undefined;
      },
    },
  ];
  // 变量区结束-----------------------------------

  // 方法区开始-----------------------------------
  // 刷新按钮
  const doRefresh = () => {
    queryUserDTO.value.pageNo = pagination.page;
    queryUserDTO.value.pageSize = pagination.pageSize;
    getUsers(queryUserDTO.value);
  };
  const pagination = usePagination(doRefresh);
  // 获取用户信息
  const getUsers = (data: QueryUserDTO) => {
    user.getUsersFq(data).then(res => {
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
    queryUserDTO.value = {
      ...tempDTO,
      createTimeStart: tempDTO.createTime ? (tempDTO.createTime as string[])[0] : undefined,
      createTimeEnd: tempDTO.createTime ? (tempDTO.createTime as string[])[1] : undefined,
    } as QueryUserDTO;
    getUsers(queryUserDTO.value);
  };
  // 重置搜索框
  const onResetSearch = () => {
    searchItems.forEach((it: any) => {
      it.reset ? it.reset() : (it.value.value = '');
    });
  };
  // 新增按钮
  const onAddItem = () => {
    formItems = addFormItem;
    actionTitle.value = '添加用户';
    modalDialogRef.value?.toggle();
    role.getRoles().then(res => {
      if (res.code == 200) {
        optionRoleItems.value.splice(0, optionRoleItems.value.length);
        res.data.data.forEach(item => {
          optionRoleItems.value.push({
            label: item.name,
            value: item.id,
          } as SelectOptionItem);
        });
      }
    });
    // 设置action为新增
    actionType.value = 'createUser';
    userDTO = createUserDTO;
  };
  // 更新按钮
  const onUpdateItem = (item: any) => {
    modalDialogRef.value?.toggle();
    formItems = updateFormItem;
    actionTitle.value = '编辑用户';
    formItems.forEach((it: any) => {
      if (it.key == 'roleIds') {
        const roles = item.roles;
        it.value.value = roles.map((it: any) => it.id);
      } else {
        const propName = item[it.key] ? item[it.key] : '';
        it.value.value = propName;
      }
    });
    role.getRoles().then(res => {
      if (res.code == 200) {
        optionRoleItems.value.splice(0, optionRoleItems.value.length);
        res.data.data.forEach(item => {
          optionRoleItems.value.push({
            label: item.name,
            value: item.id,
          } as SelectOptionItem);
        });
      }
    });
    updateUserDTO.value.id = item.id;
    userDTO = updateUserDTO;
    // 设置action为更新
    actionType.value = 'updateUser';
  };
  // 确认按钮
  const onConfirm = () => {
    if (formItems.every((it: any) => (it.validator ? it.validator() : true))) {
      // 校验通过后为DTO赋值
      formItems.forEach((item: any) => {
        if (item.value.value != '' && item.value.value != undefined && item.value.value != null) {
          userDTO.value[item.key] = item.value.value;
        }
      });
      modalDialogRef.value?.toggle();
      user[actionType.value as keyof typeof user](userDTO.value as any).then((res: any) => {
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
  // 删除
  const onDeleteItem = (item: any) => {
    Modal.confirm({
      title: '提示',
      content: '确定要删除此数据吗？',
      cancelText: '取消',
      okText: '删除',
      onOk: () => {
        user.deleteUser(item.id).then(res => {
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
    user.updateUserStatus(id, disableFlag).then(res => {
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
