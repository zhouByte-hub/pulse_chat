<template>
  <AuthLayout>
    <div class="forgot-password-container">
      <div class="forgot-password-header">
        <div class="forgot-password-logo">
          <el-icon :size="40"><Lock /></el-icon>
        </div>
        <h1 class="forgot-password-title">重置密码</h1>
        <p class="forgot-password-subtitle">请输入您的邮箱，我们将发送重置密码的链接</p>
      </div>

      <el-form
        ref="forgotPasswordFormRef"
        :model="forgotPasswordForm"
        :rules="forgotPasswordRules"
        class="forgot-password-form"
        @submit.prevent="handleSubmit"
      >
        <el-form-item prop="email">
          <el-input
            v-model="forgotPasswordForm.email"
            size="large"
            placeholder="请输入您的邮箱"
            prefix-icon="Message"
            clearable
          />
        </el-form-item>
        
        <el-form-item prop="verificationCode">
          <div class="verification-code-container">
            <el-input
              v-model="forgotPasswordForm.verificationCode"
              size="large"
              placeholder="请输入验证码"
              prefix-icon="Key"
              clearable
            />
            <el-button
              type="primary"
              size="large"
              class="send-code-btn"
              :disabled="isSendingCode || countdown > 0"
              @click="sendVerificationCode"
            >
              {{ countdown > 0 ? `${countdown}秒后重试` : '发送验证码' }}
            </el-button>
          </div>
        </el-form-item>

        <el-form-item prop="newPassword">
          <el-input
            v-model="forgotPasswordForm.newPassword"
            type="password"
            size="large"
            placeholder="请输入新密码"
            prefix-icon="Lock"
            show-password
            clearable
          />
        </el-form-item>

        <el-form-item prop="confirmPassword">
          <el-input
            v-model="forgotPasswordForm.confirmPassword"
            type="password"
            size="large"
            placeholder="请确认新密码"
            prefix-icon="Lock"
            show-password
            clearable
          />
        </el-form-item>

        <el-button
          type="primary"
          size="large"
          class="submit-btn"
          :loading="loading"
          native-type="submit"
        >
          重置密码
        </el-button>
      </el-form>

      <div class="back-to-login">
        <el-link type="primary" :underline="false" @click="$router.push('/login')">
          返回登录
        </el-link>
      </div>
    </div>
  </AuthLayout>
</template>

<script setup>
import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { Lock } from '@element-plus/icons-vue'
import AuthLayout from '@/layouts/AuthLayout.vue'
import { useResetPasswordStore } from '@/stores/resetPassword'

const resetPasswordStore = useResetPasswordStore()
const router = useRouter()
const forgotPasswordFormRef = ref()
const loading = ref(false)
const isSendingCode = ref(false)
const countdown = ref(0)
let timer = null

const forgotPasswordForm = reactive({
  email: '',
  verificationCode: '',
  newPassword: '',
  confirmPassword: ''
})

const forgotPasswordRules = {
  email: [
    { required: true, message: '请输入邮箱', trigger: 'blur' },
    { type: 'email', message: '请输入正确的邮箱格式', trigger: 'blur' }
  ],
  verificationCode: [
    { required: true, message: '请输入验证码', trigger: 'blur' },
    { len: 6, message: '验证码长度应为6位', trigger: 'blur' }
  ],
  newPassword: [
    { required: true, message: '请输入新密码', trigger: 'blur' },
    { min: 6, message: '密码至少6位', trigger: 'blur' }
  ],
  confirmPassword: [
    { required: true, message: '请确认新密码', trigger: 'blur' },
    { 
      validator: (rule, value, callback) => {
        if (value !== forgotPasswordForm.newPassword) {
          callback(new Error('两次输入的密码不一致'))
        } else {
          callback()
        }
      },
      trigger: 'blur'
    }
  ]
}

const sendVerificationCode = async () => {
  try {
    // 验证邮箱格式
    const emailValid = await forgotPasswordFormRef.value.validateField('email')
    if (!emailValid) return
    
    isSendingCode.value = true

    // 开始倒计时
    countdown.value = 60
    timer = setInterval(() => {
      countdown.value--
      if (countdown.value <= 0) {
        clearInterval(timer)
      }
    }, 1000)
    
    const response = await resetPasswordStore.sendForgetPasswordEmail(forgotPasswordForm.email)
    if (!response.success) {
      ElMessage.error(response.message)
      return
    }else{
      ElMessage.success('验证码已发送到您的邮箱')
    }
    

  } catch (error) {
    ElMessage.error('验证码发送失败，请重试')
  } finally {
    isSendingCode.value = false
  }
}

const handleSubmit = async () => {
  try {
    const valid = await forgotPasswordFormRef.value.validate()
    if (!valid) return

    loading.value = true
    
    let response = await resetPasswordStore.resetPassword({
      email: forgotPasswordForm.email,
      code: forgotPasswordForm.verificationCode,
      new_password: forgotPasswordForm.newPassword,
      check_password: forgotPasswordForm.confirmPassword,
    })
    if (!response.success) {
      ElMessage.error(response.message)
      return
    }
    
    ElMessage.success('密码重置成功')
    router.push('/login')
  } catch (error) {
    ElMessage.error('密码重置失败，请重试')
  } finally {
    loading.value = false
  }
}
</script>

<style lang="scss" scoped>
.forgot-password-container {
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

.forgot-password-header {
  text-align: center;
  margin-bottom: 40px;
}

.forgot-password-logo {
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

.forgot-password-title {
  font-size: 28px;
  color: var(--el-text-color-primary);
  margin-bottom: 10px;
  font-weight: 600;
}

.forgot-password-subtitle {
  color: var(--el-text-color-regular);
  font-size: 16px;
}

.forgot-password-form {
  margin-bottom: 30px;

  .el-form-item {
    margin-bottom: 20px;
  }
}

.submit-btn {
  width: 100%;
  height: 50px;
  font-size: 16px;
  font-weight: 600;
}

.verification-code-container {
  display: flex;
  gap: 10px;
  
  .el-input {
    flex: 1;
  }
  
  .send-code-btn {
    width: 120px;
    white-space: nowrap;
  }
}

.back-to-login {
  text-align: center;
  margin-top: 20px;
}
</style>