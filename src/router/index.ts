import type { RouteRecordRaw, Router } from 'vue-router'
import { createRouter, createWebHashHistory as createHistory } from 'vue-router'

// 固定路由  nvm.vue nrm.vue yarn.vue pnpm.vue ts.vue
export const constantRouter: RouteRecordRaw[] = [
  {
    path: '/node',
    alias: '/',
    name: 'node',
    component: () => import('@/views/node.vue')
  },
  {
    path: '/nvm',
    name: 'nvm',
    component: () => import('@/views/nvm.vue')
  },
  {
    path: '/nrm',
    name: 'nrm',
    component: () => import('@/views/nrm.vue')
  },
  {
    path: '/yarn',
    name: 'yarn',
    component: () => import('@/views/yarn.vue')
  },
  {
    path: '/pnpm',
    name: 'pnpm',
    component: () => import('@/views/pnpm.vue')
  },
  {
    path: '/ts',
    name: 'ts',
    component: () => import('@/views/ts.vue')
  }
]

const router: Router = createRouter({
  history: createHistory(),
  scrollBehavior(to, _from, savedPosition) {
    if (savedPosition && to.meta.cache) {
      return savedPosition
    }
    return { left: 0, top: 0 }
  },
  routes: [...constantRouter] as RouteRecordRaw[]
})

export default router
