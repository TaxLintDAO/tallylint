<template>
  <div class="market-banner container-xl q-mt-lg">
    <div class="row q-col-gutter-md">
      <div
        class="col-6 col-md-4 col-lg-3"
        v-for="(item, index) in cards"
        :key="index"
      >
        <q-card>
          <q-card-section>
            {{ item.title }}
          </q-card-section>
        </q-card>
      </div>
    </div>
    banner
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue"

// 定义一个数组，包含卡片的数据
const cards = ref([
  { title: "Card 1" },
  { title: "Card 2" },
  { title: "Card 3" },
  { title: "Card 4" },
])
</script>

<style lang="scss" scoped>
.market-banner {
}
</style>
