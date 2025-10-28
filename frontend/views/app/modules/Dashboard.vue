<template>
  <div class="dashboard-container">
    <!-- 由于IC一次只能查一千条交易记录，暂时先搞个提示 -->
    <q-banner
      inline-actions
      class="text-white bg-red q-mb-md"
      v-if="isTransactionTooMany"
    >
      Your single wallet transaction record is greater than 1000, can not be
      properly queried
      <template v-slot:action>
        <q-btn
          flat
          color="white"
          label="Close"
          @click="isTransactionTooMany = false"
        />
      </template>
    </q-banner>
    <!-- 为 ECharts 准备一个定义了宽高的 DOM -->
    <div class="row q-gutter-md">
      <div class="col-7" ref="echartsContainer" style="height: 400px"></div>
      <div class="col-4">
        <div class="q-pa-md" style="max-width: 300px">
          <el-date-picker
            v-model="date"
            type="daterange"
            range-separator="To"
            start-placeholder="Start date"
            end-placeholder="End date"
            :shortcuts="shortcuts"
            value-format="x"
            @change="changeDate()"
            size="large"
          />
        </div>
        <q-card flat bordered>
          <q-item>
            <q-item-section>
              <q-item-label caption> BREAKDOWN </q-item-label>
              <q-list>
                <q-item clickable v-ripple="true">
                  <q-item-section>Received</q-item-section>
                  <q-item-section side>
                    {{ convertCurrency(received) }}
                  </q-item-section>
                </q-item>
                <q-item clickable v-ripple="true">
                  <q-item-section>Sent</q-item-section>
                  <q-item-section side>
                    {{ convertCurrency(received) }}
                  </q-item-section>
                </q-item>
                <q-item clickable v-ripple="true">
                  <q-item-section>Gains</q-item-section>
                  <q-item-section side>
                    {{ convertCurrency(gains) }}
                  </q-item-section>
                </q-item>
              </q-list>
            </q-item-section>
          </q-item>
        </q-card>
      </div>
    </div>
    <div class="row">
      <div class="col-7">
        <q-card flat bordered>
          <q-item>
            <q-item-section>
              <q-item-label caption style="font-size: 1rem">
                Holdings
              </q-item-label>
              <q-table
                class="holdings"
                ref="holdings"
                :rows="rows"
                :columns="columns"
                row-key="symbol"
                column-sort-order="da"
                :rowsPerPageOptions="[0]"
                hide-pagination
                flat
              >
                <template v-slot:header="props">
                  <q-tr :props="props">
                    <q-th auto-width />
                    <q-th
                      v-for="col in props.cols"
                      :key="col.name"
                      :props="props"
                    >
                      {{ col.label }}
                    </q-th>
                  </q-tr>
                </template>

                <template v-slot:body="props">
                  <q-tr :props="props">
                    <q-td auto-width>
                      <q-btn
                        size="sm"
                        color="primary"
                        round
                        dense
                        @click="props.expand = !props.expand"
                        :icon="props.expand ? 'remove' : 'add'"
                      />
                    </q-td>
                    <q-td
                      v-for="col in props.cols"
                      :key="col.name"
                      :props="props"
                    >
                      <template
                        v-if="col.name === 'price' || col.name === 'value'"
                      >
                        {{ convertCurrency(col.value) }}
                      </template>
                      <template v-else-if="col.name === 'symbol'">
                        <div class="flex-y-center token-symbol">
                          <img
                            class="selected-icon q-mr-xs"
                            :src="props.row.logo"
                            alt="Icon"
                          />
                          {{ col.value }}
                        </div>
                      </template>
                      <template v-else> {{ col.value }}</template>
                    </q-td>
                  </q-tr>
                  <q-tr v-show="props.expand" :props="props" no-hover>
                    <q-td colspan="100%">
                      <div class="text-left">
                        <Progress
                          :wallets="wallets"
                          :symbol="props.row.symbol"
                          :price="props.row.price"
                          :totalBalance="props.row.balance"
                        />
                      </div>
                    </q-td>
                  </q-tr>
                </template>
              </q-table>
            </q-item-section>
          </q-item>
        </q-card>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { getUserCurrencyRate } from "@/api/baseCurrencies"
import { ICP_LOGO } from "@/api/constants/tokens"
import {
  getAllWalletDailyBalance,
  getDailyBalanceValue,
  getICPBalance,
  getWalletHistory,
} from "@/api/icp"
import { getICRC1Balance } from "@/api/icrc1"
import { getICPNowPrice } from "@/api/token"
import { getUserWallet } from "@/api/user"
import Progress from "@/components/Progress.vue"
import type { TableColumn } from "@/types/model"
import type { Wallet, WalletHistory } from "@/types/user"
import { convertCurrency } from "@/utils/currencies"
import { numberToFixed, processNumber } from "@/utils/math"
import { showMessageError } from "@/utils/message"
import type { EChartsType } from "echarts"
import * as echarts from "echarts"
import type { QTable } from "quasar"
import { computed, onMounted, ref } from "vue"

const echartsContainer = ref<null>(null)
let chart = <EChartsType>{} // 如果使用ref会导致无法显示tooltip，例如数据不会显示。
const date = ref("")
const totalHistory = ref<WalletHistory[]>([])
const received = ref(0)
const sent = ref(0)
const gains = ref(0)
const isTransactionTooMany = ref(false)

const shortcuts = [
  {
    text: "Last 12 months",
    value: () => {
      const end = new Date()
      const start = new Date()
      start.setMonth(end.getMonth() - 12)
      return [start, end]
    },
  },
  {
    text: new Date().getFullYear().toString(),
    value: () => {
      const end = new Date()
      const start = new Date(end.getFullYear(), 0, 1)
      return [start, end]
    },
  },
  {
    text: (new Date().getFullYear() - 1).toString(),
    value: () => {
      const end = new Date(new Date().getFullYear() - 1, 11, 31)
      const start = new Date(new Date().getFullYear() - 1, 0, 1)
      return [start, end]
    },
  },
  {
    text: (new Date().getFullYear() - 2).toString(),
    value: () => {
      const end = new Date(new Date().getFullYear() - 2, 11, 31)
      const start = new Date(new Date().getFullYear() - 2, 0, 1)
      return [start, end]
    },
  },
]

const columns: TableColumn[] = [
  {
    name: "symbol",
    required: true,
    label: "Assets",
    field: "symbol",
    align: "left",
  },
  {
    name: "balance",
    required: true,
    label: "Balance",
    sortable: true,
    field: "balance",
    align: "left",
  },
  // {
  //   name: "cost",
  //   required: true,
  //   label: "Cost",
  //   sortable: true,
  //   field: "cost",
  //   align: "left",
  // },
  {
    name: "price",
    required: true,
    label: "Price",
    sortable: true,
    field: "price",
    align: "left",
  },
  {
    name: "value",
    required: true,
    label: "Value",
    sortable: true,
    field: "value",
    align: "left",
  },
]

const wallets = ref<Wallet[]>([])
const rate = ref(1)
const holdings = ref<QTable>()
//将 tokenSummary 转换为适合 q-table 的行数据格式
const rows = computed(() => {
  return Object.keys(tokenSummary.value).map((symbol) => ({
    symbol: symbol,
    price: tokenSummary.value[symbol].price,
    logo: tokenSummary.value[symbol].logo,
    balance: tokenSummary.value[symbol].totalBalance,
    value: tokenSummary.value[symbol].totalValue,
  }))
})

const tokenSummary = computed(() => {
  const summary: Record<
    string,
    { logo: string; price: number; totalBalance: number; totalValue: number }
  > = {}
  wallets.value.forEach((wallet) => {
    wallet.tokens.forEach((token) => {
      if (!summary[token.symbol]) {
        summary[token.symbol] = {
          price: 0,
          logo: "",
          totalBalance: 0,
          totalValue: 0,
        }
      }
      summary[token.symbol].price = token.price
      summary[token.symbol].logo = token.logo
      summary[token.symbol].totalBalance = numberToFixed(
        summary[token.symbol].totalBalance + token.balance,
        8,
      )
      summary[token.symbol].totalValue = processNumber(
        summary[token.symbol].totalValue + token.value,
      )
    })
  })
  return summary
})

onMounted(async () => {
  //如果获取汇率的过程中发生了错误或者返回的结果中没有汇率，则使用原值
  rate.value = (await getUserCurrencyRate()).rate || rate.value
  //默认以value列作为排序，最高的value排最上面
  holdings.value?.sort("value")
  //TODO 当用户第一次登录调用此方法会报错，这个问题暂时没有影响，可以先不管，速度优先。
  getWallet() // 在初始化完成后调用
  initECharts()
  // getNFTCollections(
  //   "34a6b-pl5tx-mpxgn-bnucu-tiwis-qxbqu-pilo5-lmnt7-fa7yk-ng22p-yae",
  // )
})

const getBalance = async (
  address: string,
  principal: string,
  walletName: string,
) => {
  //获取用户当前钱包资产
  const balance = await getICPBalance(address)
  const res = await getICPNowPrice()
  //需要用到price作为计算，这里还没法将它直接转换为字符串的货币符号
  const ICPPrice = processNumber(res * rate.value)
  const tokens = [
    {
      symbol: "ICP",
      logo: ICP_LOGO,
      balance: balance,
      price: ICPPrice,
      value: processNumber(balance * ICPPrice),
    },
  ]
  if (principal) {
    const icrc1 = await getICRC1Balance(principal)
    icrc1.map((token) => {
      tokens.push(token)
    })
  }
  wallets.value.push({
    address: address,
    name: walletName,
    tokens: tokens,
  })
}

const getWallet = async () => {
  const res = await getUserWallet(false)
  if (res.Ok && res.Ok[0]) {
    //清空钱包，以免出现重复的问题
    wallets.value.length = 0
    for (const walletInfo of res.Ok) {
      //TODO 这里作为BREAKDOWN的数值，有bug，多个钱包的资产总值没有计算，而下面的echarts图表没有bug，已经计算了。
      //将用户的每个钱包地址下的交易记录查出来，并总和到一起
      const walletHistory = await getWalletHistory(walletInfo.address)
      getBalance(
        walletInfo.address,
        walletInfo.principal_id[0],
        walletInfo.name,
      )

      totalHistory.value = totalHistory.value.concat(walletHistory.history)
      //IC一次性查询不能超过一千条
      isTransactionTooMany.value = walletInfo.transactions >= 1000
    }
    const walletDailyBalance = await getAllWalletDailyBalance(res.Ok)
    let timestamps = Object.keys(walletDailyBalance).sort()
    const balances = await getDailyBalanceValue(walletDailyBalance)
    // 按选定的日期区间过滤记录
    if (date.value) {
      totalHistory.value = totalHistory.value.filter(
        (item) =>
          item.timestamp >= Number(date.value[0]) &&
          item.timestamp <= Number(date.value[1]),
      )
      //为echarts图表过滤
      timestamps = timestamps.filter((timestamp) => {
        const currentDate = Number(new Date(timestamp))
        return (
          currentDate >= Number(date.value[0]) &&
          currentDate <= Number(date.value[1])
        )
      })
    }
    getDetail()
    // 基于准备好的dom，初始化echarts实例

    chart.hideLoading()
    // 绘制图表
    chart.setOption({
      title: {
        text: `Synchronized Total History (${res.Ok.length} wallets)`,
      },
      series: [
        {
          name: "Value ($)",
          type: "line",
          symbol: "none",
          smooth: true,
          sampling: "lttb",
          itemStyle: {
            color: "rgb(255, 70, 131)",
          },
          areaStyle: {
            color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
              {
                offset: 0,
                color: "rgb(255, 158, 68)",
              },
              {
                offset: 1,
                color: "rgb(255, 70, 131)",
              },
            ]),
          },
          data: balances,
        },
      ],
      xAxis: { data: timestamps },
    })
  } else {
    showMessageError("please add wallet for dashboard")
  }
}
const getDetail = () => {
  if (totalHistory.value) {
    totalHistory.value.forEach((transaction) => {
      if (transaction.t_type === "RECEIVE") {
        received.value += transaction.amount * transaction.price
      } else if (transaction.t_type === "SEND") {
        sent.value += transaction.amount * transaction.price
      }
    })

    //保留小数点
    received.value = Number((received.value * rate.value).toFixed(2))
    sent.value = Number((sent.value * rate.value).toFixed(2))
    //gain暂且使用这种简单的方式计算
    gains.value = Number(
      ((received.value - sent.value) * rate.value).toFixed(2),
    )
  }
}
// 初始化 ECharts 实例
const initECharts = () => {
  chart = echarts.init(echartsContainer.value)
  // 配置图表选项
  const option = {
    tooltip: {
      trigger: "axis",
      position: function (pt) {
        return [pt[0], "10%"]
      },
    },
    title: {
      left: "center",
      text: "Total History (0 wallets)",
    },
    toolbox: {
      feature: {
        dataZoom: {
          yAxisIndex: "none",
        },
        saveAsImage: {},
      },
    },
    xAxis: {
      type: "category",
      boundaryGap: false,
      data: [],
    },
    yAxis: {
      type: "value",
      boundaryGap: false, //是否留白
    },
    dataZoom: [
      {
        type: "inside",
        start: 0,
        end: 100,
      },
      {
        //启用图表下方的进度条，方便拖放
        start: 0,
        end: 100,
      },
    ],
  }

  // 使用 setOption 方法将配置应用到图表
  chart.setOption(option)
  chart.showLoading()
}
const changeDate = () => {
  console.log("date", date.value)
  chart.showLoading()
  getWallet()
}
</script>

<style lang="scss" scoped>
.token-symbol {
  font-size: 16px;
}
//启用展开行之后会导致qtable的行之间有隐藏的展开行，导致单纯的奇偶数无法选择正确的行，只能进行奇数行之间的隔行变色
.holdings tbody tr:nth-of-type(4n-1) {
  background: rgba(0, 0, 0, 0.02);
}
</style>
