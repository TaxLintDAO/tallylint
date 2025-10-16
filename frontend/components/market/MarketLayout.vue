<template>
  <div>
    <marketNav v-if="shouldShowNav" />
    <router-view />
  </div>
</template>

<script setup lang="ts">
import marketNav from "@/components/market/MarketNavigator.vue"
import { useRoute } from "vue-router"
const route = useRoute()
// 需要隐藏导航条的页面
const hideNavPages = ["/market/xxx"] // 未来可以扩展

const shouldShowNav = !hideNavPages.includes(route.path)
</script>

<style lang="scss" scoped></style>
