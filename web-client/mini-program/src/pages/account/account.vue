<template>
  <view class="account-area">
    <u-form :model="account">
      <u-form-item borderBottom labelWidth="150" labelPosition="left" label="昵称" prop="nickname">
        <u-input v-model="account.nickname" border="none"></u-input>
      </u-form-item>
      <u-form-item borderBottom labelWidth="150" labelPosition="left" label="邮箱" prop="email">
        <u-input v-model="account.email" border="none"></u-input>
      </u-form-item>
      <u-form-item borderBottom labelWidth="150" labelPosition="left" label="电话" prop="phoneNumber">
        <u-input v-model="account.phoneNumber" border="none"></u-input>
      </u-form-item>
      <u-form-item borderBottom labelWidth="150" labelPosition="left" label="性别" prop="gender" @click="showPicker">
        <u-picker
          :columns="columns"
          @confirm="confirmGender"
          @cancel="cancelGender"
          itemHeight="80"
          title="请选择性别"
          :show="showPickerPanel"
        ></u-picker>
        <u-input v-model="account.gender" border="none" disabled disabledColor="transparent"></u-input>
      </u-form-item>
      <u-form-item borderBottom labelWidth="150" labelPosition="left" label="国家" prop="country">
        <u-input v-model="account.country" border="none"></u-input>
      </u-form-item>
      <u-form-item borderBottom labelWidth="150" labelPosition="left" label="省份" prop="province">
        <u-input v-model="account.province" border="none"></u-input>
      </u-form-item>
      <u-form-item labelWidth="150" labelPosition="left" label="城市" prop="city">
        <u-input v-model="account.city" border="none"></u-input>
      </u-form-item>
    </u-form>
  </view>
  <view class="save-btn">
    <u-button text="保存修改" size="large" @click="save"></u-button>
  </view>
</template>

<script setup lang="ts">
  import { saveAccount } from '@/api/account';
  import type { Account } from '@/models/account';
  import type { UserInfo } from '@/models/login';
  import { useUserStore } from '@/store/user';
  import { ref } from 'vue';
  const user = useUserStore();
  const account = ref(<Account>{ ...user.userInfo });
  const showPickerPanel = ref(false);
  const columns = ref([['男', '女']]);
  const showPicker = () => {
    showPickerPanel.value = true;
  };
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const confirmGender = (e: any) => {
    account.value.gender = e.value[0];
    showPickerPanel.value = false;
  };
  const cancelGender = () => {
    showPickerPanel.value = false;
  };

  const save = () => {
    saveAccount(account.value).then(res => {
      if (res.code == '200') {
        user.userInfo = <UserInfo>{ ...account.value };
      }
      uni.navigateBack();
    });
  };
</script>

<style scoped lang="scss">
  .account-area {
    margin-top: 30rpx;
    background-color: white;
    padding: 0 50rpx;
    height: 100%;
  }
  .save-btn {
    margin-top: 50rpx;
  }
</style>
