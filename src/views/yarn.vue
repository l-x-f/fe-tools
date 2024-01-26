<template>
  <div class="page">{{ data }}</div>
</template>

<script lang="ts" setup>
import { reactive, toRefs } from 'vue'

defineOptions({
  name: 'AppYarn'
})

const state = reactive({ data: 'yarn' })
const { data } = toRefs(state)
</script>

<style lang="scss" scoped>
.page {
  background-color: #fff;
}
</style>
