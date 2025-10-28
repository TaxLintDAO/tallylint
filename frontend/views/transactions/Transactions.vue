<template>
  <div class="transactions-container">
    <div class="column">
      <div class="row items-center justify-between q-mb-sm">
        <div class="text-h4 row flex-y-center">
          Transactions
          <span class="transaction-number">
            {{ transactionAmount }}
          </span>
        </div>
        <div>
          <q-btn color="primary" @click="openDialog('add')"
            >Add Transaction</q-btn
          >
        </div>
      </div>
      <div class="header row q-mb-md justify-between">
        <div class="header-left row q-gutter-md">
          <q-select
            v-model="selectedWallet"
            use-chips
            multiple
            option-label="name"
            :options="wallets"
            label="All Wallets"
          >
            <template v-slot:option="scope">
              <q-item v-bind="scope.itemProps">
                <q-item-section avatar>
                  <img
                    class="head-icon"
                    src="@/assets/dfinity.svg"
                    alt="NNS Icon"
                  />
                </q-item-section>
                <q-item-section>
                  <q-item-label>{{ scope.opt.name }}</q-item-label>
                  <q-item-label caption v-if="scope.opt.last_sync_time !== 0">
                    Synced
                  </q-item-label>
                </q-item-section>
              </q-item>
            </template>
          </q-select>
          <q-select
            multiple
            use-chips
            v-model="selectTokens"
            :options="tokens"
            style="max-width: 205px"
            label="Token"
          >
            <template v-slot:selected-item="scope">
              <q-chip
                removable
                @remove="scope.removeAtIndex(scope.index)"
                :tabindex="scope.tabindex"
                class="q-ma-none"
              >
                <img
                  v-if="scope"
                  class="selected-icon q-mr-xs"
                  :src="scope.opt.meta.logo"
                  alt="Icon"
                />
                {{ scope.opt.symbol }}
              </q-chip>
            </template>
            <template v-slot:option="scope">
              <q-item v-bind="scope.itemProps">
                <q-item-section avatar>
                  <img
                    class="head-icon"
                    :src="scope.opt.meta.logo"
                    alt="Icon"
                  />
                </q-item-section>
                <q-item-section>
                  <q-item-label>{{ scope.opt.symbol }}</q-item-label>
                  <q-item-label caption>{{ scope.opt.name }}</q-item-label>
                </q-item-section>
              </q-item>
            </template></q-select
          >
          <q-select
            use-chips
            multiple
            v-model="type"
            :options="typeOptions"
            label="Type"
          />
          <q-select
            use-chips
            multiple
            v-model="tag"
            :options="tagOptions"
            label="Tag"
          />
          <q-select
            multiple
            use-chips
            v-model="manual"
            :options="manualOptions"
            label="Manual"
          />
        </div>
        <div class="header-right row q-gutter-md flex-y-center">
          <el-date-picker
            v-model="date"
            type="daterange"
            range-separator="To"
            start-placeholder="Start date"
            end-placeholder="End date"
            :shortcuts="shortcuts"
            value-format="x"
          />
          <q-select
            rounded
            outlined
            v-model="sort"
            :options="sortOptions"
            label="Sort by"
          />
          <q-select
            rounded
            outlined
            v-model="pageSize"
            :options="pagesizeOptions"
            label="Per page"
          />
        </div>
      </div>
      <div v-if="showLoading">
        <q-spinner-cube size="xl" color="primary" />
      </div>
      <div v-else-if="maxPage == 0">
        <span
          >No transactions already synchronized, please synchronize transaction
          history of your wallet on Wallet page</span
        >
      </div>
      <div v-else>
        <q-list bordered separator>
          <template
            v-for="(transactions, date, index) in paginatedGroups"
            :key="index"
          >
            <q-item>
              <q-item-section>
                <q-item-label class="text-h6" header>{{ date }}</q-item-label>
              </q-item-section>
            </q-item>
            <q-item
              v-for="transaction in transactions"
              :key="Number(transaction.id)"
              clickable
              style="padding: 20px"
            >
              <!-- transaction log -->
              <div class="row items-center" style="width: 100%">
                <div class="col">
                  <q-icon
                    v-if="transaction.t_type === 'SEND'"
                    class="text-red-5"
                    size="md"
                    name="arrow_upward"
                  />
                  <q-icon
                    v-if="transaction.t_type === 'RECEIVE'"
                    class="text-green-6"
                    size="md"
                    name="arrow_downward"
                  />
                  <span v-if="transaction.tag.length > 0">
                    {{ transaction.tag[0] }}
                  </span>
                  <span v-else> {{ transaction.t_type }}</span>
                  <q-badge
                    v-if="transaction.manual"
                    rounded
                    class="q-ml-sm"
                    color="primary"
                    label="M"
                  />

                  <br />
                  {{ showCustomTimezoneTime(transaction.timestamp) }}
                </div>
                <div class="col">
                  <div class="flex-y-center">
                    <img
                      v-if="
                        snsInfo.find(
                          (sns) =>
                            sns.symbol === transaction.details.currency.symbol,
                        )
                      "
                      class="head-icon"
                      :src="
                        snsInfo.find(
                          (sns) =>
                            sns.symbol === transaction.details.currency.symbol,
                        )?.meta.logo
                      "
                      alt="Icon"
                    />
                    <img
                      v-else
                      class="head-icon"
                      src="@/assets/dfinity.svg"
                      alt="NNS Icon"
                    />
                    <div class="q-ml-xs">
                      {{
                        getTransactionWalletName(
                          transaction.t_type,
                          transaction.details,
                          wallets,
                        )
                      }}
                    </div>
                  </div>
                  <span v-if="transaction.t_type === 'SEND'">-</span>
                  {{ transaction.details.currency.symbol }}
                  {{ transaction.details.amount }}
                  <br />
                  <span v-if="transaction.t_type === 'SEND'">
                    <CurrencyUSD :amount="transaction.details.cost" />
                    cost basis
                  </span>
                </div>
                <div class="col">
                  <q-icon
                    v-if="transaction.t_type === 'RECEIVE'"
                    size="md"
                    class="text-green-6"
                    name="arrow_back"
                  />
                  <q-icon
                    v-else
                    size="md"
                    class="text-red-5"
                    name="arrow_forward"
                  />
                </div>
                <div class="col">
                  {{
                    showUsername(
                      "",
                      transaction.t_type === "RECEIVE"
                        ? transaction.details.from
                        : transaction.details.to || "",
                    )
                  }}
                  <a
                    @click="
                      toTransactionDetail(
                        transaction.details.currency.symbol,
                        transaction.hash,
                      )
                    "
                  >
                    <q-icon name="open_in_new" />
                  </a>
                  <br />
                  <span>
                    ≈
                    <CurrencyUSD :amount="transaction.details.value" />
                    <q-tooltip
                      anchor="top middle"
                      self="bottom middle"
                      :offset="[10, 10]"
                    >
                      Market price by
                      {{
                        transaction.details.currency.symbol === "ICP"
                          ? ROSETTA_WEBSITE
                          : ICRC_WEBSITE
                      }}
                      <br />
                      ${{ transaction.details.price }} /
                      {{ transaction.details.currency.symbol }}
                    </q-tooltip>
                  </span>
                  <b
                    v-if="transaction.t_type === 'SEND'"
                    :class="{
                      'text-green-6': transaction.details.profit > 0,
                      'text-red-5': transaction.details.profit < 0,
                    }"
                  >
                    ·
                    <CurrencyUSD :amount="transaction.details.profit" />
                    profit
                  </b>
                </div>
                <div class="col">
                  <q-icon size="sm" name="more_vert"
                    ><q-menu>
                      <q-list style="min-width: 100px">
                        <div v-if="transaction.t_type === 'SEND'">
                          <q-item clickable v-close-popup="true">
                            <q-item-section
                              @click="tagTransaction(transaction.id, 'Gift')"
                            >
                              Tag As Gift
                            </q-item-section>
                          </q-item>
                          <q-item clickable v-close-popup="true">
                            <q-item-section
                              @click="tagTransaction(transaction.id, 'Lost')"
                            >
                              Tag As Lost
                            </q-item-section>
                          </q-item>
                        </div>
                        <div v-if="transaction.t_type === 'RECEIVE'">
                          <q-item clickable v-close-popup="true">
                            <q-item-section
                              @click="tagTransaction(transaction.id, 'Reward')"
                            >
                              Tag As Reward
                            </q-item-section>
                          </q-item>
                          <q-item clickable v-close-popup="true">
                            <q-item-section
                              @click="tagTransaction(transaction.id, 'Airdrop')"
                            >
                              Tag As Airdrop
                            </q-item-section>
                          </q-item>
                        </div>
                        <q-separator />
                        <q-item
                          v-if="transaction.tag.length > 0"
                          clickable
                          v-close-popup="true"
                        >
                          <q-item-section @click="removeTag(transaction.id)">
                            Remove Tag
                          </q-item-section>
                        </q-item>
                        <q-item clickable v-close-popup="true">
                          <q-item-section
                            @click="openDialog('edit', transaction)"
                          >
                            Edit
                          </q-item-section>
                        </q-item>
                        <q-item
                          v-if="transaction.manual"
                          clickable
                          v-close-popup="true"
                        >
                          <q-item-section
                            @click="deleteTransaction(transaction.id)"
                          >
                            Delete
                          </q-item-section>
                        </q-item>
                      </q-list>
                    </q-menu></q-icon
                  >
                </div>
              </div>
            </q-item>
          </template>
        </q-list>
        <q-pagination
          v-model="currentPage"
          :max="maxPage"
          direction-links
          boundary-links
          class="justify-center q-mt-md"
        />
      </div>
    </div>
    <q-dialog v-model="dialogVisible">
      <q-card style="min-width: 450px">
        <q-card-section>
          <q-form @submit="onSubmit" ref="form" class="q-gutter-md">
            <q-card flat bordered class="my-card">
              <q-card-section>
                <div class="text-h6 row justify-between q-mb-md">
                  <div class="text-h6">
                    {{ isEdit ? "Edit Transaction" : "Add Transaction" }}
                  </div>
                  <div class="text-grey text-caption row items-center">
                    {{ isEdit ? "Auto Import" : "Manual" }}
                  </div>
                </div>
                <q-select
                  v-model="transactionWallet"
                  filled
                  :options="wallets"
                  label="Select Wallet *"
                  :rules="[(val) => !!val || 'Please select wallet']"
                  :disable="!transaction.manual && isEdit"
                >
                  <template v-slot:selected-item="scope">
                    <q-item style="padding-left: 0">
                      <q-item-section avatar>
                        <img
                          class="head-icon"
                          src="@/assets/dfinity.svg"
                          alt="NNS Icon"
                        />
                      </q-item-section>
                      <q-item-section>
                        <q-item-label>{{ scope.opt.name }}</q-item-label>
                      </q-item-section>
                      <q-item-section side>
                        <q-item-label caption>{{
                          showUsername("", scope.opt.address)
                        }}</q-item-label>
                      </q-item-section>
                    </q-item>
                  </template>
                  <template v-slot:option="scope">
                    <q-item v-bind="scope.itemProps">
                      <q-item-section avatar>
                        <img
                          class="head-icon"
                          src="@/assets/dfinity.svg"
                          alt="NNS Icon"
                        />
                      </q-item-section>
                      <q-item-section>
                        <q-item-label>{{ scope.opt.name }}</q-item-label>
                        <q-item-label
                          caption
                          v-if="scope.opt.last_sync_time !== 0"
                        >
                          Synced
                        </q-item-label>
                      </q-item-section>
                      <q-item-section side>
                        <q-item-label caption>{{
                          showUsername("", scope.opt.address)
                        }}</q-item-label>
                      </q-item-section>
                    </q-item>
                  </template>
                </q-select>
                <q-input
                  filled
                  label="Amount of tokens"
                  type="number"
                  v-model.number="transaction.details.amount"
                  class="q-mb-md"
                  :disable="!transaction.manual && isEdit"
                >
                  <template v-slot:before>
                    <q-select
                      filled
                      map-options
                      option-label="symbol"
                      :option-value="
                        (opt) => {
                          return {
                            decimals: opt.decimals,
                            symbol: opt.symbol,
                          }
                        }
                      "
                      v-model="transaction.details.currency"
                      :options="tokenList"
                      label="Token"
                      style="min-width: 100px"
                      :disable="!transaction.manual && isEdit"
                    >
                      <template v-slot:option="scope">
                        <q-item v-bind="scope.itemProps">
                          <q-item-section avatar>
                            <img
                              class="head-icon"
                              :src="scope.opt.meta.logo"
                              alt="Icon"
                            />
                          </q-item-section>
                          <q-item-section>
                            <q-item-label>{{ scope.opt.symbol }}</q-item-label>
                            <q-item-label caption>{{
                              scope.opt.name
                            }}</q-item-label>
                          </q-item-section>
                        </q-item>
                      </template>
                    </q-select>
                  </template>
                </q-input>
                <q-input
                  filled
                  label="Token Price"
                  type="number"
                  v-model.number="transaction.details.price"
                  :disable="!transaction.manual && isEdit"
                >
                  <template v-slot:prepend> $ </template>
                </q-input>
              </q-card-section>

              <q-separator inset />

              <q-card-section>
                <div class="text-h6 row justify-between q-mb-md">
                  <div class="row items-center">
                    <q-icon
                      v-if="transaction.t_type === 'SEND'"
                      class="text-red-5"
                      size="md"
                      name="arrow_downward"
                    />
                    <q-icon
                      v-else
                      class="text-green-6"
                      size="md"
                      name="arrow_upward"
                    />
                    <q-select
                      style="padding-bottom: 0"
                      filled
                      v-model="transaction.t_type"
                      :options="typeOptions"
                      label="Type *"
                      :rules="[
                        (val) =>
                          (val && val.length > 0) ||
                          'Please select transaction type',
                      ]"
                      :disable="!transaction.manual && isEdit"
                    />
                  </div>
                  <div class="text-grey text-caption row items-center">
                    {{ isEdit ? "Auto Import" : "Manual" }}
                  </div>
                </div>
                <q-input
                  filled
                  label="Target Address *"
                  v-model.trim="targetAddress"
                  :rules="[
                    (val) =>
                      (val && val.length > 0) || 'Please enter target address',
                    (val) =>
                      (val &&
                        val.length > 0 &&
                        (val.length === 63 || val.length === 64)) ||
                      'Please enter Account ID or Principal ID',
                  ]"
                  :disable="!transaction.manual && isEdit"
                />
              </q-card-section>
            </q-card>
            <el-date-picker
              v-model="transaction.timestamp"
              type="datetime"
              placeholder="Transaction Datetime *"
              value-format="x"
              :rules="[
                (val) =>
                  (val && val.length > 0) ||
                  'Please enter the transaction datetime',
              ]"
              :disable="!transaction.manual && isEdit"
            />
            <q-input
              outlined
              label="Transaction Hash *"
              v-model.trim="transaction.hash"
              class="q-mb-md"
              :rules="[
                (val) =>
                  (val && val.length > 0) ||
                  'Please enter the transaction hash',
              ]"
              :disable="!transaction.manual && isEdit"
            />
            <q-input
              outlined
              label="Description"
              type="textarea"
              v-model="transaction.comment"
              class="q-mb-md"
            />
            <div class="q-gutter-sm justify-end flex">
              <q-btn flat label="Cancel" v-close-popup="true" />
              <q-btn
                v-if="isEdit"
                :loading="loading"
                label="Update"
                type="submit"
                color="primary"
              />
              <q-btn
                v-else
                :loading="loading"
                label="Save"
                type="submit"
                color="primary"
              />
            </div>
          </q-form>
        </q-card-section>
      </q-card>
    </q-dialog>
  </div>
</template>

<script lang="ts" setup>
import { ICRC_WEBSITE, ROSETTA_WEBSITE } from "@/api/constants/ic"
import { ICP_LOGO } from "@/api/constants/tokens"
import { getSNSInfoCache } from "@/api/sns"
import {
  addManualTransaction,
  deleteSyncedTransactions,
  editUserTransaction,
  getUserWallet,
  removeTransactionTag,
  setTransactionTag,
} from "@/api/user"
import CurrencyUSD from "@/components/CurrencyUSD.vue"
import type { ICRC1Info, SyncedTransaction } from "@/types/tokens"
import type { WalletTag } from "@/types/user"
import { showUsername } from "@/utils/avatars"
import { showCustomTimezoneDate, showCustomTimezoneTime } from "@/utils/date"
import { confirmDialog } from "@/utils/dialog"
import { showMessageError, showMessageSuccess } from "@/utils/message"
import { getTokenList } from "@/utils/storage"
import {
  getAllSyncedTransactions,
  getTransactionWalletName,
} from "@/utils/syncedTransactions"
import type { QForm } from "quasar"
import { computed, onMounted, ref, watch } from "vue"
import { useRoute } from "vue-router"

const route = useRoute()
const wid = route.params.wid
const transactionsList = ref<SyncedTransaction[]>([])
const transaction = ref({
  id: 0n,
  wid: 0n,
  tag: [],
  hash: "",
  memo: "",
  walletName: "",
  t_type: "SEND",
  comment: "",
  address: "",
  timestamp: 0,
  details: {
    to: "",
    fee: 0,
    status: "",
    ledgerCanisterId: "",
    value: 0,
    cost: 0,
    from: "",
    currency: { decimals: 8, symbol: "ICP" },
    profit: 0,
    price: 0,
    amount: 0,
  },
  manual: false,
  principal_id: [],
})
const selectTokens = ref<ICRC1Info[]>([])
const tokens = getTokenList() || []
const type = ref<string[]>([])
const typeOptions = ["SEND", "RECEIVE"]
const tag = ref<string[]>([])
const tagOptions = ["Reward", "Lost", "Gift", "Airdrop"]
const date = ref("") //采用这个方便判定为空
const manual = ref<string[]>([])
const manualOptions = ["Manual"]
const sort = ref({ label: "Recent", value: "date-desc" })
const sortOptions = [
  { label: "Recent", value: "date-desc" },
  { label: "Oldest first", value: "date-asc" },
  //TODO 获取利润排序的分组有点问题，因为是按日期分的页，所以同一日期的顺序会乱
  { label: "Highest gains", value: "profit-desc" },
  { label: "Lowest gains", value: "profit-asc" },
]
const tokenList = [
  {
    decimals: 8,
    symbol: "ICP",
    name: "Internet Computer",
    meta: {
      logo: ICP_LOGO,
    },
  },
]

const form = ref<QForm | null>(null)
const selectedWallet = ref<WalletTag[]>([]) //用户选择的钱包
const wallets = ref<WalletTag[]>([])
const transactionWallet = ref<WalletTag | null>(null) //编辑，增加dialog 中用户选择的钱包
const targetAddress = ref() //编辑，增加dialog 中用户输入的目标地址
const snsInfo = ref<ICRC1Info[]>([])

const showLoading = ref(false)
const loading = ref(false)
const dialogVisible = ref(false)
const isEdit = ref(false)

const currentPage = ref(1)
const maxPage = ref(1)
const pageSize = ref(10)
const transactionAmount = ref(0)
const pagesizeOptions = ["5", "10", "25", "50"]
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
//TODO 按每一日，将数据分组，如果使用按时间排序以外的方法排序，会造成同一天里包含错误排序的数据
const groupedTransactions = (
  transactions: SyncedTransaction[],
): {
  [date: string]: SyncedTransaction[]
} => {
  const groups = {}
  // if (sort.value.value !== "date-desc" && sort.value.value !== "date-asc") {
  //   return groups
  // }
  transactions.forEach((transaction) => {
    const date = showCustomTimezoneDate(transaction.timestamp)
    if (!groups[date]) {
      groups[date] = []
    }
    groups[date].push(transaction)
  })
  return groups
}
//先分页，再分组。
const paginatedGroups = computed(
  (): {
    [date: string]: SyncedTransaction[]
  } => {
    const start = (currentPage.value - 1) * pageSize.value
    const end = start + pageSize.value
    let paginatedData = transactionsList.value
    // 分页之前先按选定的日期区间过滤记录
    if (date.value) {
      paginatedData = paginatedData.filter(
        (item) =>
          item.timestamp >= Number(date.value[0]) &&
          item.timestamp <= Number(date.value[1]),
      )
    }
    paginatedData = paginatedData.filter(
      (item) =>
        //筛选type
        (type.value.length === 0 || type.value.includes(item.t_type)) &&
        //筛选tag
        (tag.value.length === 0 ||
          item.tag.some((tagItem) => tag.value.includes(tagItem))) &&
        //筛选manual
        (manual.value.length === 0 ||
          (item.manual && manual.value.includes("Manual"))) &&
        // 筛选currency.symbol
        (selectTokens.value.length === 0 ||
          selectTokens.value.findIndex(
            (token) => token.symbol === item.details.currency.symbol,
          ) !== -1),
    )
    //在slice分页之前处理页码和数量
    maxPage.value = Math.ceil(paginatedData.length / pageSize.value)
    transactionAmount.value = paginatedData.length
    paginatedData = paginatedData.slice(start, end)
    return groupedTransactions(paginatedData)
  },
)

// 监听 date, type, tag 和 manual 变化，如果有任何一个发生变化，强制重新计算 paginatedGroups
watch([selectTokens, date, type, tag, manual], () => {
  paginatedGroups.value // 触发 paginatedGroups 的重新计算
})
//监听排序，被选择的钱包数值变化，触发查询
watch([sort, selectedWallet], () => {
  getSelectedWalletHistory(selectedWallet.value)
})

onMounted(() => {
  init()
})

const init = () => {
  getWallets().then(() => {
    if (wid) {
      // 如果 route wid 存在，则单独查询wid相关的交易记录，如果查不到相关钱包id，即传入空值
      selectedWallet.value = wallets.value.filter((item) => {
        return item.id === Number(wid)
      })
    } else {
      // 如果 wid 不存在，则默认使用canister查询IC数据库全部内容
      getSelectedWalletHistory(wallets.value)
    }
  })
  getSNSInfoCache().then((snses) => {
    snsInfo.value = snses
    tokenList.push(...snses)
  })
}

const getWallets = async () => {
  showLoading.value = true
  const userWallets = await getUserWallet(false)
  console.log("userWallets", userWallets)
  const mapToWallet = (wallet: {
    id: bigint
    name: string
    address: string
    principal_id: string[]
    from: string
    last_sync_time: number
  }) => ({
    id: Number(wallet.id),
    name: wallet.name,
    address: wallet.address,
    principal: wallet.principal_id,
    from: wallet.from,
    last_sync_time: wallet.last_sync_time,
  })
  if (userWallets.Ok) {
    wallets.value = userWallets.Ok.map(mapToWallet)
  } else {
    showMessageError("get Wallets Error")
  }
}

const getSelectedWalletHistory = async (selectedWallets: WalletTag[]) => {
  showLoading.value = true
  currentPage.value = 1
  let targetWallets: WalletTag[]
  //如果没有选择任何钱包，则查询所有钱包
  selectedWallets.length !== 0
    ? (targetWallets = selectedWallets)
    : (targetWallets = wallets.value)
  getAllSyncedTransactions(0, 0, [sort.value.value], targetWallets)
    .then((res) => {
      console.log("getWalletHistory", res)
      if (res.total && res.total != 0) {
        transactionsList.value = res.transactions
        maxPage.value = Math.ceil(res.total / pageSize.value)
        transactionAmount.value = res.total
      }
    })
    .finally(() => {
      showLoading.value = false
    })
}

const openDialog = (action: string, itemInfo?: any) => {
  //将状态置空
  targetAddress.value = ""
  transactionWallet.value = null
  transaction.value = {
    id: 0n,
    wid: 0n,
    tag: [],
    hash: "",
    memo: "",
    walletName: "",
    t_type: "SEND",
    comment: "",
    address: "",
    timestamp: 0,
    details: {
      to: "",
      fee: 0,
      status: "",
      ledgerCanisterId: "",
      value: 0,
      cost: 0,
      from: "",
      currency: { decimals: 8, symbol: "ICP" },
      profit: 0,
      price: 0,
      amount: 0,
    },
    manual: false,
    principal_id: [],
  }

  if (action === "edit" && itemInfo) {
    isEdit.value = true
    transaction.value = { ...itemInfo }
    //由于transaction没有memo字段，不满足edit方法的参数，所以这里添加
    transaction.value.memo = ""
    transaction.value.address = ""
    //理论上来说item里manual属性会覆盖transaction里的manual属性，所以这里不对manual做修改
    if (itemInfo.t_type === "SEND") {
      targetAddress.value = itemInfo.details.to
      transactionWallet.value =
        wallets.value.find(
          (wallet) => wallet.address === itemInfo.details.from,
        ) || null
    } else if (itemInfo.t_type === "RECEIVE") {
      targetAddress.value = itemInfo.details.from
      transactionWallet.value =
        wallets.value.find(
          (wallet) => wallet.address === itemInfo.details.to,
        ) || null
    }
  } else {
    //不为edit就是add
    isEdit.value = false
    transaction.value.manual = true
  }
  dialogVisible.value = true
}

const onSubmit = async () => {
  loading.value = true
  console.log("trans", transactionWallet.value)
  const validationSuccess = await form.value?.validate()

  try {
    if (validationSuccess) {
      if (transactionWallet.value) {
        const { details } = transaction.value
        const { address } = transactionWallet.value
        //根据type的类型，将details的from和to填入，不能新建任何变量，保证解构的变量details能修改引用的transaction.value值
        details.from =
          transaction.value.t_type === "SEND" ? address : targetAddress.value
        details.to =
          transaction.value.t_type === "SEND" ? targetAddress.value : address
      }

      if (isEdit.value) {
        await editTransaction()
      } else {
        await addTransaction()
      }
      dialogVisible.value = false
    } else {
      // 数据验证失败
      // 用户至少输入了一个无效值
    }
  } catch (error) {
    console.error("onSubmitError", error)
    showMessageError((error as Error).toString())
  } finally {
    loading.value = false
  }
}

const addTransaction = async () => {
  console.log("addTransaction", wallets.value)
  transaction.value.wid = BigInt(wallets.value[0].id)
  // addedTransaction.timestamp *= MILI_PER_SECOND
  console.log("addTransaction", transaction.value)
  const res = await addManualTransaction(transaction.value)
  console.log("res", res)
  if (res.Ok) {
    showMessageSuccess("Add Transaction Success")
    getSelectedWalletHistory(selectedWallet.value)
  }
  return
}

const editTransaction = async () => {
  console.log("editTransaction", transaction.value)
  const res = await editUserTransaction(transaction.value)
  console.log("res", res)
  if (res.Ok) {
    showMessageSuccess("Edit Transaction Success")
  }
  return
}

const tagTransaction = (transactionId: bigint | number, tag: string) => {
  console.log("tag", tag)
  setTransactionTag(transactionId, tag).then((res) => {
    if (res.Ok) {
      showMessageSuccess(`Tag ${tag} set success`)
      getSelectedWalletHistory(selectedWallet.value)
    }
  })
}

const removeTag = (transactionId: bigint | number) => {
  confirmDialog({
    title: "Delete Transaction Tag",
    message: "Are you sure delete this tag? Deleted tag can't restore",
    okMethod: () => {
      removeTransactionTag(transactionId).then((res) => {
        if (res.Ok) {
          showMessageSuccess(`Tag delete success`)
          getSelectedWalletHistory(selectedWallet.value)
        }
      })
    },
  })
}

const deleteTransaction = (transactionId: bigint | number) => {
  confirmDialog({
    title: "Delete Transaction",
    message:
      "Are you sure delete this transaction? Deleted transaction can't restore",
    okMethod: () => {
      deleteSyncedTransactions(transactionId).then((res) => {
        console.log("deleteSyncedTransactions", res)
        if (res.Ok) {
          showMessageSuccess("delete transaction success")
          init()
        }
      })
    },
  })
}
const toTransactionDetail = (symbol: string, hash: string) => {
  if (symbol === "ICP") {
    window.open(
      "https://dashboard.internetcomputer.org/transaction/" + hash,
      "_blank",
    )
  } else {
    const sns = snsInfo.value.find((sns) => sns.symbol === symbol)
    if (sns) {
      window.open(
        `https://dashboard.internetcomputer.org/sns/${sns.canisters.root}/transaction/${hash}`,
        "_blank",
      )
    } else {
      showMessageError("failed get token Info, please try again later.")
    }
  }
}
</script>

<style lang="scss">
.transactions-container {
  .transaction-number {
    font-size: 14px;
    line-height: 15px;
    color: #6c757d;
    border: 1px solid silver;
    border-radius: 10px;
    padding: 4px 8px;
    margin-left: 10px;
  }
  .header {
    .q-select {
      min-width: 150px;
    }
  }
}
</style>
