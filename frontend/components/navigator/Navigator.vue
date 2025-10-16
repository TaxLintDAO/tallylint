<template>
  <div class="navigator-container">
    <q-toolbar class="navbar">
      <img
        alt="logo"
        src="@/assets/tallylint.png"
        class="cursor-pointer"
        @click="onHome"
      />

      <div class="flex-y-center">
        <!-- <router-link :to="'/market'">
          <q-btn flat rounded label="Market" class="q-mr-md" no-caps />
        </router-link> -->

        <q-separator dark vertical />
        <q-btn
          color="primary"
          class="login-button"
          @click="onLogin()"
          :loading="loading"
          no-caps
        >
          Launch app
        </q-btn>
      </div>
    </q-toolbar>
  </div>
</template>
<script lang="ts" setup>
import { IdentityInfo, initAuth, signIn } from "@/api/auth"
import { setCurrentIdentity } from "@/api/canister_pool"
import { useUserStore } from "@/stores/user"
import { ref } from "vue"
import { useRouter } from "vue-router"

const router = useRouter()
const userStore = useUserStore()
const onHome = () => router.push("/")
// 与 II 认证相关的信息
const signedIn = ref(false) // 是否登录

const loading = ref(false)
const onLogin = async () => {
  const auth = await initAuth()
  loading.value = true
  //TODO 先不使用登录缓存，有点问题
  // if (!auth.info) {
  //检查用户是否已登录，未登录就登录
  signIn(auth.client) // 理论上有链接对象才会进入这个方法
    .then((ii) => {
      signedIn.value = true
      auth.info = ii
      loginSuccess(ii)
    })
    .catch((e) => {
      console.error("e", e)
    })
    .finally(() => {
      loading.value = false
    })
  // } else {
  //   //存在auth.info，说明用户已登录，不需要再登录
  //   loginSuccess(auth.info)
  // }
}

const loginSuccess = (ii: IdentityInfo) => {
  // 保存登录状态到actor，方便调用
  setCurrentIdentity(ii.identity, ii.principal)
  // 保存 principal 到状态
  userStore.setPrincipal(ii.principal).then(() => {
    //直接跳转到应用中，在应用里获取userInfo，加快速度。
    router.push({
      path: "/app",
    })
  })
}
</script>
<style lang="scss" scoped>
.navigator-container {
  width: 100%;
  height: 80px;
  display: flex;
  background-color: #ffffff !important;
  box-shadow: 0px 0px 10px 10px rgb(36 51 54 / 5%);
  .navbar {
    width: 1200px;
    margin: auto;
    padding: 0;
    display: flex;
    justify-content: space-between;
    align-items: center;
    img {
      height: 51px;
    }
  }
}
</style>
