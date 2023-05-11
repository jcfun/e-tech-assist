<template>
  <div class="captcha">
    <a-input v-model="value" allow-clear size="large" :placeholder="placeholder" :max-length="4">
      <template #prefix>
        <IconExclamationCircle />
      </template>
    </a-input>
    <div class="captcha-img" @click="$emit('click-img')">
      <a-image :preview="false" :src="captchaImg" height="35" width="90" />
    </div>
  </div>
</template>

<script setup lang="ts">
  import { computed } from 'vue';
  const props = withDefaults(
    defineProps<{
      modelValue: string | null;
      captchaImg: string;
      placeholder: string;
    }>(),
    {
      modelValue: null,
      captchaImg: 'http://file.urainstar.top/error.png',
      placeholder: '',
    },
  );
  const emit = defineEmits<{
    (e: 'update:modelValue', value: string | null): void;
    (e: 'click-img'): void;
  }>();
  const value = computed({
    get: () => props.modelValue,
    set: val => emit('update:modelValue', val),
  });
</script>

<style scoped lang="scss">
  .captcha {
    display: flex;
    flex-direction: row;
    .arco-input-wrapper {
      margin-right: 5px;
    }
    .captcha-img {
      display: flex;
      justify-content: center;
      .arco-image {
        :deep(.arco-image-img) {
          max-height: 35px;
          max-width: none;
        }
        :deep(.arco-image-overlay) {
          .arco-image-error {
            justify-content: start;
          }
        }
      }
    }
  }
</style>
