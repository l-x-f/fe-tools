<script setup lang="ts">
import { onUnmounted, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { debounce } from 'lodash-es'
import { listen } from '@tauri-apps/api/event'

interface ListenEvent {
  event: 'clickMenuItem'
  id: number
  payload: string
  windowLabel: 'main'
}

const router = useRouter()
const unListen = ref()

const handleEvent = debounce((event: ListenEvent) => {
  event.payload && router.push(`/${event.payload}`)
})

onMounted(async () => {
  unListen.value = await listen('clickMenuItem', handleEvent as any)
})

onUnmounted(() => {
  unListen.value()
})
</script>

<template><router-view /></template>
