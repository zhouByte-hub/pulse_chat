<template>
  <AuthLayout>
    <div class="login-container">
      <div class="login-header">
        <div class="login-logo">
          <el-icon :size="40"><ChatDotRound /></el-icon>
        </div>
        <h1 class="login-title">欢迎回来</h1>
        <p class="login-subtitle">登录您的 Pulse Chat 账户</p>
      </div>

      <el-form
        ref="loginFormRef"
        :model="loginForm"
        :rules="loginRules"
        class="login-form"
        @submit.prevent="handleLogin"
      >
        <el-form-item prop="username">
          <el-input
            v-model="loginForm.username"
            size="large"
            placeholder="请输入用户名或邮箱"
            prefix-icon="User"
            clearable
          />
        </el-form-item>

        <el-form-item prop="password">
          <el-input
            v-model="loginForm.password"
            type="password"
            size="large"
            placeholder="请输入密码"
            prefix-icon="Lock"
            show-password
            clearable
          />
        </el-form-item>

        <div class="form-options">
          <el-checkbox v-model="loginForm.rememberMe">记住我</el-checkbox>
          <el-link type="primary" :underline="false">忘记密码？</el-link>
        </div>

        <el-button
          type="primary"
          size="large"
          class="login-btn"
          :loading="authStore.loading"
          native-type="submit"
        >
          登录
        </el-button>
      </el-form>

      <el-divider>使用以下方式登录</el-divider>

      <div class="social-login">
        <el-button circle class="social-btn" @click="handleSocialLogin('wechat')">
          <el-icon><Message /></el-icon>
        </el-button>
        <el-button circle class="social-btn" @click="handleSocialLogin('qq')">
          <el-icon><ChatLineRound /></el-icon>
        </el-button>
        <el-button circle class="social-btn" @click="handleSocialLogin('github')">
          <el-icon><Platform /></el-icon>
        </el-button>
      </div>

      <div class="register-link">
        还没有账户？
        <el-link type="primary" :underline="false" @click="$router.push('/register')">
          立即注册
        </el-link>
      </div>
    </div>
  </AuthLayout>
</template>

<script setup>
import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { ChatDotRound, Message, ChatLineRound, Platform } from '@element-plus/icons-vue'
import AuthLayout from '@/layouts/AuthLayout.vue'
import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const authStore = useAuthStore()
const loginFormRef = ref()

const loginForm = reactive({
  username: '',
  password: '',
  rememberMe: false
})

const loginRules = {
  username: [
    { required: true, message: '请输入用户名或邮箱', trigger: 'blur' },
    { min: 3, message: '用户名至少3个字符', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, message: '密码至少6位', trigger: 'blur' }
  ]
}

const handleLogin = async () => {
  try {
    const valid = await loginFormRef.value.validate()
    if (!valid) return

    const result = await authStore.login({
      username: loginForm.username,
      password: loginForm.password,
      rememberMe: loginForm.rememberMe
    })

    console.log(result)
    if (result.success) {
      ElMessage.success('登录成功')
      router.push('/chat')
    } else {
      ElMessage.error(result.message)
    }
  } catch (error) {
    ElMessage.error('登录失败，请重试')
  }
}

const handleSocialLogin = (platform) => {
  ElMessage.info(`正在跳转到${platform}登录...`)
  // 实现第三方登录逻辑
}
</script>

<style lang="scss" scoped>
.login-container {
  width: 100%;
  max-width: 400px;
  margin: 0 auto;
  padding: 40px;
  background: white;
  border-radius: 16px;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
  position: relative;
  overflow: hidden;

  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 4px;
    background: linear-gradient(90deg, var(--el-color-primary), var(--el-color-success));
  }
}

.login-header {
  text-align: center;
  margin-bottom: 40px;
}

.login-logo {
  width: 80px;
  height: 80px;
  background: var(--el-color-primary);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto 20px;
  color: white;
  box-shadow: 0 8px 16px rgba(64, 158, 255, 0.3);
}

.login-title {
  font-size: 28px;
  color: var(--el-text-color-primary);
  margin-bottom: 10px;
  font-weight: 600;
}

.login-subtitle {
  color: var(--el-text-color-regular);
  font-size: 16px;
}

.login-form {
  margin-bottom: 30px;

  .el-form-item {
    margin-bottom: 20px;
  }
}

.form-options {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 30px;
}

.login-btn {
  width: 100%;
  height: 50px;
  font-size: 16px;
  font-weight: 600;
}

.social-login {
  display: flex;
  justify-content: center;
  gap: 15px;
  margin: 30px 0;
}

.social-btn {
  width: 50px;
  height: 50px;
  border-radius: 50%;
  
  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(64, 158, 255, 0.2);
  }
}

.register-link {
  text-align: center;
  color: var(--el-text-color-regular);
  font-size: 15px;
}
</style>