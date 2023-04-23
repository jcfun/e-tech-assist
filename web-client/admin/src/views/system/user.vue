<template>
  <TableBody>
    <template #header>
      <TableHeader ref="tableHeaderRef" :show-filter="false">
        <template #table-config>
          <AddButton @add="onAddItem" />
        </template>
      </TableHeader>
    </template>
    <template #default>
      <a-table
        :loading="table.tableLoading"
        :data="table.dataList"
        :pagination="false"
        :rowKey="rowKey"
        column-resizable
        table-layout-fixed
        :scroll="{ y: table.tableHeight }"
        @selection-change="onSelectionChange"
      >
        <template #columns>
          <a-table-column
            v-for="item of tableColumns"
            :key="item.key"
            :align="item.align"
            :title="(item.title as string)"
            :data-index="(item.key as string)"
            :fixed="item.fixed"
          >
            <template v-if="item.key === 'index'" #cell="{ rowIndex }">
              {{ rowIndex + 1 }}
            </template>
            <template v-else-if="item.key === 'gender'" #cell="{ record }">
              <a-tag :color="record.gender === '1' ? 'green' : record.gender === '0' ? 'red' : 'gray'">
                {{ record.gender === '1' ? '男' : record.gender === '0' ? '女' : '未知' }}
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
            <template v-else-if="item.key === 'address'" #cell="{ record }">
              {{ `${record.country ? record.country : ''} ${record.province ? record.province : ''} ${record.city ? record.city : ''}` }}
            </template>
            <template v-else-if="item.key === 'disableFlag'" #cell="{ record }">
              <a-tag color="blue" size="small" v-if="record.disableFlag === '0'">正常</a-tag>
              <a-tag color="red" size="small" v-else>禁用</a-tag>
            </template>
            <template v-else-if="item.key === 'actions'" #cell="{ record }">
              <a-button type="text" size="mini" @click="onUpdateItem(record)">编辑</a-button>
              <a-button type="text" size="mini" @click="onDeleteItem(record)">删除</a-button>
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
          :disabled="accountDisable && item.key === 'account'"
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
  import type { QueryUserDTO } from '@/api/types/user';
  import { usePagination, useRowKey, useRowSelection, useTable, useTableColumn, useTableHeight } from '@/hooks/table';
  import { Message, Modal } from '@arco-design/web-vue';
  import { getCurrentInstance, nextTick, onMounted, ref } from 'vue';
  import type { UserDTO } from '@/api/types/user';
  import type { FormItem, ModalDialogType, SelectOptionItem } from '@/components/types';
  import role from '@/api/requests/role';

  const table = useTable();
  const rowKey = useRowKey('id');
  const pagination = usePagination(doRefresh);
  const { onSelectionChange } = useRowSelection();

  // 变量区开始-----------------------------------
  // 操作弹出框标题
  const actionTitle = ref('添加用户');
  const userDTO = ref({} as UserDTO);
  const modalDialogRef = ref<ModalDialogType | null>(null);
  const optionRoleItems = ref([] as SelectOptionItem[]);
  // 添加还是修改
  let actionType = ref('add');
  // 修改时禁止修改账号
  let accountDisable = ref(false);

  const tableColumns = useTableColumn([
    table.indexColumn,
    {
      title: '账号',
      key: 'account',
      dataIndex: 'account',
      show: true,
    },
    {
      title: '手机号',
      key: 'phoneNumber',
      dataIndex: 'phoneNumber',
      show: true,
    },
    {
      title: '邮箱',
      key: 'email',
      dataIndex: 'email',
      show: true,
    },
    {
      title: '昵称',
      key: 'nickname',
      dataIndex: 'nickname',
      show: true,
    },
    {
      title: '性别',
      key: 'gender',
      dataIndex: 'gender',
      show: true,
    },
    {
      title: '头像',
      key: 'avatarUrl',
      dataIndex: 'avatarUrl',
      show: true,
    },
    {
      title: '地址',
      key: 'address',
      dataIndex: 'city',
      show: true,
    },
    {
      title: '描述',
      key: 'description',
      dataIndex: 'description',
      show: true,
    },
    {
      title: '登录时间',
      key: 'lastLoginTime',
      dataIndex: 'lastLoginTime',
      show: true,
    },
    {
      title: '登录IP',
      key: 'lastLoginIp',
      dataIndex: 'lastLoginIp',
      show: true,
    },
    {
      title: '状态',
      key: 'disableFlag',
      dataIndex: 'disableFlag',
      show: true,
    },
    {
      title: '操作',
      key: 'actions',
      dataIndex: 'actions',
      show: true,
    },
  ]);

  const formItems = [
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
      label: '密码',
      key: 'password',
      value: ref(''),
      type: 'input',
      placeholder: '请输入用户密码',
    },
    {
      label: '是否启用',
      key: 'disableFlag',
      value: ref(''),
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
        this.value.value = undefined;
      },
    },
    {
      label: '用户描述',
      key: 'description',
      value: ref(''),
      type: 'textarea',
      placeholder: '请输入用户描述',
    },
  ] as FormItem[];
  // 变量区结束-----------------------------------

  // 方法区开始-----------------------------------
  // 获取用户信息
  const getUsers = () => {
    const dto = {
      pageNo: pagination.page,
      pageSize: pagination.pageSize,
    } as QueryUserDTO;
    user.getUsersFq(dto).then(res => {
      table.handleSuccess(res?.data?.data);
      pagination.setTotalSize(res?.data?.total_page);
    });
  };

  function doRefresh() {
    getUsers();
  }

  function onAddItem() {
    actionTitle.value = '添加用户';
    modalDialogRef.value?.toggle();
    role.getRole().then(res => {
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
    actionType.value = 'createUser';
    accountDisable.value = false;
  }

  function onUpdateItem(item: any) {
    actionTitle.value = '编辑用户';
    modalDialogRef.value?.toggle();
    console.log(item);
    nextTick(() => {
      formItems.forEach(it => {
        if (it.key == 'roleIds') {
          const roles = item.roles;
          it.value.value = roles.map((it: any) => it.id);
        } else {
          const propName = item[it.key];
          it.value.value = propName;
        }
      });
      role.getRole().then(res => {
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
      userDTO.value.id = item.id;
    });
    actionType.value = 'updateUser';
    accountDisable.value = true;
  }

  const onConfirm = () => {
    if (formItems.every(it => (it.validator ? it.validator() : true))) {
      formItems.forEach(item => {
        if (item.value.value != '' && item.value.value != undefined && item.value.value != null) {
          userDTO.value[item.key] = item.value.value;
        }
      });
      modalDialogRef.value?.toggle();
      user[actionType.value as keyof typeof user](userDTO.value as any).then((res: any) => {
        if (res.code == 200) {
          Message.success(res.msg);
          formItems.forEach(it => {
            if (it.reset) {
              it.reset();
            } else {
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

  function onDeleteItem(item: any) {
    Modal.confirm({
      title: '提示',
      content: '确定要删除此数据吗？',
      cancelText: '取消',
      okText: '删除',
      onOk: () => {
        Message.success('数据删除成功');
        table.dataList.splice(table.dataList.indexOf(item), 1);
      },
    });
  }
  onMounted(async () => {
    table.tableHeight.value = await useTableHeight(getCurrentInstance());
    doRefresh();
  });
  // 方法区结束-----------------------------------
</script>
