<template>
  <div class="predict-container q-pt-lg">
    <div class="container">
      <q-form @submit="onSubmit" ref="form" class="q-gutter-md">
        <div class="col-7" ref="echartsContainer" style="height: 400px"></div>
        <q-select
          filled
          v-model="predict.model"
          :options="models"
          label="Predict Model Version"
        />
        <q-select
          filled
          v-model="predict.model"
          :options="models"
          label="Predict Model Parameters"
        />
        <q-input
          filled
          v-model="predict.model"
          label="Wallet Address *"
          hint="Input Principal ID"
          lazy-rules
        />
        <q-input
          filled
          disable
          label="Wallet Account ID"
          v-model="predict.model"
        />
        <q-input
          filled
          v-model="predict.model"
          label="Wallet Name *"
          hint="Identify your wallet quickly"
          lazy-rules
          :rules="[
                (val: string) => (val && val.length > 0) || 'Please input wallet name',
              ]"
        />
        <div class="q-gutter-sm justify-end flex">
          <q-btn flat label="Cancel" v-close-popup="true" />
          <q-btn
            :loading="loading"
            label="Submit"
            type="submit"
            color="primary"
          />
        </div>
      </q-form>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { EChartsType } from "echarts"
import * as echarts from "echarts"
import type { QForm } from "quasar"
import { ref } from "vue"

const form = ref<QForm | null>(null)
let chart = <EChartsType>{}
const loading = ref(false)
const predict = ref({ model: "LSTM" })
const models = ["LSTM"]

// 初始化 ECharts 实例
// const initECharts = () => {
//   chart = echarts.init(echartsContainer.value)
//   // 配置图表选项
//   const option = {
//     tooltip: {
//       trigger: "axis",
//       position: function (pt) {
//         return [pt[0], "10%"]
//       },
//     },
//     title: {
//       left: "center",
//       text: "Total History (0 wallets)",
//     },
//     toolbox: {
//       feature: {
//         dataZoom: {
//           yAxisIndex: "none",
//         },
//         saveAsImage: {},
//       },
//     },
//     xAxis: {
//       type: "category",
//       boundaryGap: false,
//       data: [],
//     },
//     yAxis: {
//       type: "value",
//       boundaryGap: false, //是否留白
//     },
//     dataZoom: [
//       {
//         type: "inside",
//         start: 0,
//         end: 100,
//       },
//       {
//         //启用图表下方的进度条，方便拖放
//         start: 0,
//         end: 100,
//       },
//     ],
//   }

//   // 使用 setOption 方法将配置应用到图表
//   chart.setOption(option)
//   chart.showLoading()
// }

const onSubmit = async () => {
  loading.value = true

  loading.value = false
}
</script>

<style lang="scss" scoped></style>
