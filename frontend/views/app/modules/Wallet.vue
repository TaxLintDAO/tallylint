<template xmlns:v-slot="http://www.w3.org/1999/XSL/Transform">
  <!-- <q-banner rounded class="text-white bg-orange-10">
    In order to support IRCR1 tokens, TallyLint no longer supports wallet in the
    form of Account ID, only Principal ID entered into wallet are supported now.
    <br />
    Wallet already entered are not affected, but wallet without Principal ID
    cannot support IRCR1 tokens
  </q-banner> -->
  <div class="wallet-container">
    <div class="row">
      <div class="q-gutter-sm">
        <q-btn color="primary" @click="openDialog('add')">Add Wallet</q-btn>
        <q-btn color="secondary" @click="openChildDialog()" icon="cached">
          Sync All Wallets
        </q-btn>
        <q-btn
          flat
          color="primary"
          label="How to get principal id "
          icon="lightbulb_outline"
          :href="DOCS_URL + WALLET_HELP"
          target="_blank"
        />
        <SupportedTokens
          ref="supportedTokensRef"
          :method="syncAllWallet"
          :userWallets="wallets"
          :loading="syncLoading"
        />
      </div>

      <q-space />
      <q-input dense debounce="300" v-model="filter" placeholder="Search">
        <template v-slot:append>
          <q-icon name="search" />
        </template>
      </q-input>
    </div>
    <q-table
      grid
      :loading="tableLoading"
      :rows="wallets"
      :columns="columns"
      selection="multiple"
      v-model:selected="selected"
      :filter="filter"
      row-key="address"
    >
      <template v-slot:item="props">
        <div
          class="q-pa-xs col-xs-12 col-sm-6 col-md-4 col-lg-3 grid-style-transition"
        >
          <q-card>
            <q-card-section>
              <q-card-section>
                <div class="row justify-between items-center">
                  <div class="flex q-gutter-xs">
                    <img
                      class="head-icon"
                      src="@/assets/dfinity.svg"
                      alt="NNS Icon"
                    />
                    <div class="text-h6">{{ props.row.name }}</div>
                  </div>
                  <q-btn flat icon="more_vert">
                    <q-menu>
                      <q-list style="min-width: 100px">
                        <q-item clickable v-close-popup="true">
                          <q-item-section
                            @click="openDialog('edit', props.row)"
                          >
                            Edit
                          </q-item-section>
                        </q-item>
                        <q-item clickable v-close-popup="true">
                          <q-item-section @click="deleteWallet(props.row.id)">
                            Delete
                          </q-item-section>
                        </q-item>
                      </q-list>
                    </q-menu>
                  </q-btn>
                </div>
                <div class="sync">
                  <div v-if="props.row.last_sync_time === 0n">
                    <q-badge color="red"> NO SYNCED </q-badge>
                  </div>
                  <div v-else>
                    <q-badge color="blue"> SYNCED </q-badge>
                    {{
                      distanceFromCurrentDate(
                        Number(props.row.last_sync_time) / MILI_PER_SECOND,
                      )
                    }}
                  </div>
                </div>
              </q-card-section>
              <q-list>
                <!-- principal只有有值才显示 -->
                <q-item
                  v-if="props.row.principal_id.length > 0"
                  style="padding-bottom: 8px"
                >
                  <q-item-section>
                    <q-item-label> Principal </q-item-label>
                    <q-item-label caption>
                      <router-link
                        :to="'/app/transactions/' + Number(props.row.id)"
                      >
                        {{ props.row.principal_id[0] }}
                      </router-link>
                    </q-item-label>
                  </q-item-section>
                </q-item>
                <!-- 上面已经显示过wallet name，所以这里不需要再显示name -->
                <q-item
                  v-for="col in props.cols"
                  :key="col.name"
                  :class="{ 'hidden-row': col.name === 'name' }"
                >
                  <q-item-section>
                    <q-item-label>{{ col.label }}</q-item-label>
                    <q-item-label v-if="col.name === 'lastTransaction'" caption>
                      {{
                        distanceFromCurrentDate(props.row.last_transaction_time)
                      }}
                    </q-item-label>
                    <q-item-label v-else caption>{{ col.value }}</q-item-label>
                  </q-item-section>
                </q-item>
              </q-list>
            </q-card-section>
          </q-card>
        </div>
      </template>
    </q-table>
    <q-dialog v-model="walletDialogVisible">
      <q-card style="min-width: 350px">
        <q-card-section>
          <div class="text-h6">
            {{ isEdit ? "Edit Your Wallet" : "Your Wallet" }}
          </div>
        </q-card-section>
        <q-card-section class="q-pt-none">
          <q-form @submit="onSubmit" ref="walletForm" class="q-gutter-md">
            <q-input
              filled
              :disable="isEdit"
              v-model="address"
              label="Wallet Address *"
              hint="Input Principal ID"
              lazy-rules
              :rules="[
                (val: string) =>
                  (val && val.length > 0 && isPrincipal(val)) ||
                  'Please input Wallet Principal ID',
                (val: string) =>
                  (val && !wallets.some((item) => item.address === val)) ||
                  isEdit ||
                  'Can not add this wallet, principal id duplicated',
              ]"
            />
            <q-input
              filled
              v-if="addressIsPrincipal"
              disable
              label="Wallet Account ID"
              v-model="wallet.address"
            />
            <q-select
              filled
              v-model="wallet.from"
              :options="froms"
              label="From"
            />
            <q-input
              filled
              v-model="wallet.name"
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
                v-if="isEdit"
                :loading="loading"
                label="Update"
                type="submit"
                color="primary"
              />
              <q-btn
                v-else
                :loading="loading"
                label="Submit"
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
import { DOCS_URL, WALLET_HELP } from "@/api/constants/docs"
import { MILI_PER_SECOND } from "@/api/constants/ic"
import {
  addUserWallet,
  deleteUserWallet,
  editUserWallet,
  fetchAllSyncTransactions,
  getUserWallet,
  syncWallet,
} from "@/api/user"
import SupportedTokens from "@/components/SupportedTokens.vue"
import type { WalletInfo, syncWalletParam } from "@/types/user"
import { isPrincipal, p2a } from "@/utils/common"
import { distanceFromCurrentDate } from "@/utils/date"
import { confirmDialog } from "@/utils/dialog"
import { showMessageSuccess, showResultError } from "@/utils/message"
import type { QForm } from "quasar"
import { onMounted, ref, watch } from "vue"

const supportedTokensRef = ref<InstanceType<typeof SupportedTokens>>()
const columns = [
  {
    name: "address",
    required: true,
    label: "Account ID",
    field: "address",
  },
  { name: "from", label: "From", field: "from" },
  { name: "name", label: "Name", field: "name" },
  { name: "transactions", label: "Transactions", field: "now_transactions" },
  {
    name: "syncedTransactions",
    label: "Synced Transactions",
    field: (row: { transactions: any }) => row.transactions,
  },
  {
    name: "lastTransaction",
    label: "Last Transaction",
    field: "last_transaction_time",
  },
]
const froms = ["NNS", "Plug", "Stoic", "AstorMe", "OISY"]
const walletDialogVisible = ref(false)
const loading = ref(false)
const tableLoading = ref(false)
const syncLoading = ref(false)
const filter = ref("") // 搜索框
const selected = ref([]) // 当前选中的对象们
const address = ref("") // 当前用户输入的地址，可能是principal ID，也可能是account ID
const addressIsPrincipal = ref(false) // 是否是principal，关系到某些字段的显示
const isEdit = ref(false) // dialog是否是edit功能，否就是add功能

const wallet = ref({
  id: 0n,
  address: "",
  principal_id: [] as string[], // 无值就用[]，而不是[""]，不然opt类型会报错
  from: "NNS",
  name: "",
  now_transactions: 0,
  transactions: 0,
  last_transaction_time: 0,
  last_sync_time: 0,
})
const walletPrototype = {
  id: 0n,
  address: "",
  principal_id: [] as string[],
  from: "NNS",
  name: "",
  now_transactions: 0,
  transactions: 0,
  last_transaction_time: 0,
  last_sync_time: 0,
}
const walletForm = ref<QForm | null>(null)
const wallets = ref<WalletInfo[]>([])

onMounted(() => {
  getWallets(false)
})

const syncAllWallet = async () => {
  syncLoading.value = true
  let syncTransactionArray: syncWalletParam[] = []
  // 创建一个 Promise 数组，用于存放每个 getICPTransactions() 的 Promise
  const promises = wallets.value.map(async (row, index) => {
    const resArray = await fetchAllSyncTransactions({
      id: Number(row.id),
      address: row.address,
      principal: row.principal_id,
      name: row.name,
      from: row.from,
    })
    //将钱包数据同步
    console.log(`getWalletTransactions No.${index + 1}: `, resArray)
    syncTransactionArray.push({ walletId: row.id, history: resArray })
  })
  // 使用 Promise.all() 等待所有的请求完成
  await Promise.all(promises).catch((error) => {
    //如果失败，将loading取消
    syncLoading.value = false
  })
  console.log("transactionInfo", syncTransactionArray)
  syncWallet(syncTransactionArray)
    .then((res) => {
      console.log("syncAllWallet:", res)
      if (res.Ok) {
        getWallets(true)
        showMessageSuccess("Synchronize all wallets successfully")
      }
    })
    .catch((error) => {
      console.error("syncAllWallet Error:", error)
    })
    .finally(() => {
      syncLoading.value = false
    })
}

// 识别用户输入的地址属于principal ID还是account ID
watch(address, () => {
  identifyAddress()
})

// 识别用户输入的地址属于principal ID还是account ID
const identifyAddress = () => {
  addressIsPrincipal.value = isPrincipal(address.value)
  if (addressIsPrincipal.value) {
    wallet.value.address = p2a(address.value)
    wallet.value.principal_id = [address.value]
  } else {
    wallet.value.address = address.value
  }
}

const getWallets = (isRefresh: boolean) => {
  tableLoading.value = true
  //执行add，delete操作后刷新缓存，其他查询操作则不需要刷新缓存。
  getUserWallet(isRefresh)
    .then((res) => {
      console.log("getUserWallet", res)
      if (res.Ok) {
        wallets.value = res.Ok
        for (const row of wallets.value) {
          try {
            //TODO 感觉可以用缓存
            fetchAllSyncTransactions({
              id: Number(row.id),
              address: row.address,
              principal: row.principal_id,
              name: "",
              from: "",
            }).then((res) => {
              // 将查询得到的transactions绑定回原数组中的now_transactions，表明现在的交易数有多少
              row.now_transactions = res.length
              row.last_transaction_time = res[0].timestamp
            })
          } catch (error) {
            // 处理错误情况
            console.error(`query ${row.address} transactons error: `, error)
          }
        }
      }
    })
    .finally(() => {
      tableLoading.value = false
    })
}

const onSubmit = async () => {
  loading.value = true
  const validationSuccess = await walletForm.value?.validate()
  if (validationSuccess) {
    if (isEdit.value) {
      await editWallet()
    } else {
      await addWallet()
    }
    walletDialogVisible.value = false
  } else {
    // 数据验证失败
    // 用户至少输入了一个无效值
  }
  loading.value = false
}

const openDialog = (action: string, itemInfo?: WalletInfo) => {
  if (action === "edit" && itemInfo) {
    isEdit.value = true
    address.value = itemInfo.address
    wallet.value = { ...itemInfo }
  } else {
    //不为edit就是add
    isEdit.value = false
    address.value = ""
    wallet.value = { ...walletPrototype }
  }
  walletDialogVisible.value = true
}

const addWallet = async () => {
  const { address, name, from, principal_id } = wallet.value
  const res = await addUserWallet(
    address.trim(),
    name.trim(),
    from,
    principal_id,
  )
  if (res.Ok) {
    wallets.value.push({ ...wallet.value })
    wallet.value = { ...walletPrototype }
    walletDialogVisible.value = false
    getWallets(true)
  } else {
    showResultError(res)
  }
}

const editWallet = async () => {
  const { id, from, name } = wallet.value
  const res = await editUserWallet(id, from, name)
  if (res.Ok) {
    getWallets(true)
  } else {
    showResultError(res)
  }
}

const deleteWallet = (walletId: bigint) => {
  confirmDialog({
    title: "Delete Wallet",
    message:
      "Are you sure delete this wallet? Delete wallet will clear this wallet history info",
    okMethod: (data) => {
      deleteUserWallet(walletId).then((res) => {
        if (res.Ok) {
          getWallets(true)
          showMessageSuccess("delete wallet success")
        }
      })
    },
  })
}
const openChildDialog = () => {
  //打开子组件弹窗
  if (supportedTokensRef.value) {
    supportedTokensRef.value.openDialog()
  }
}
</script>

<style lang="scss">
.wallet-container {
  .q-list .q-item {
    padding-top: 0px;
    padding-bottom: 0px;
  }
  .grid-style-transition {
    transition: transform 0.28s, background-color 0.28s;
  }
  .q-card__section .q-card__section--vert {
    padding-bottom: 0px;
  }
  .head-icon {
    width: 32px !important;
  }
  .text-caption {
    font-size: 0.8rem;
  }
  .sync {
    color: rgba(0, 0, 0, 0.54);
    font-size: 13px;
  }
  .q-table__top {
    padding: 0;
  }
  .hidden-row {
    display: none;
  }
}
</style>
