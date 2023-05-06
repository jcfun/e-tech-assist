<template>
  <div class="redirect-container"></div>
</template>

<script lang="ts">
  import { defineComponent } from 'vue';

  export default defineComponent({
    created() {
      const { params, query } = this.$route;
      console.log('params ===> ', params);
      const { path } = params;
      this.$router.replace({
        path: '/' + (typeof path === 'string' ? path : path.join('/')),
        query,
      });
    },
  });
</script>
