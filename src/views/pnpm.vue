<template>
  <div class="page">{{ data }}</div>
</template>

<script lang="ts" setup>
import { reactive, toRefs } from 'vue'

defineOptions({
  name: 'AppPnpm'
})

const state = reactive({ data: 'pnpm' })
const { data } = toRefs(state)
</script>

<style lang="scss" scoped>
.page {
  background-color: #fff;
}
</style>
