import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import axios from 'axios'

export const useAuthStore = defineStore('auth', () => {
  const user = ref(null)
  const token = ref(localStorage.getItem('token') || null)
  const loading = ref(false)

  const isAuthenticated = computed(() => !!token.value)

  // 设置认证头
  const setAuthHeader = (authToken) => {
    if (authToken) {
      axios.defaults.headers.common['Authorization'] = `Bearer ${authToken}`
    } else {
      delete axios.defaults.headers.common['Authorization']
    }
  }

  // 登录
  const login = async (credentials) => {
    loading.value = true
    try {
      const response = await axios.post('/api/user/login', credentials)
      
      token.value = response.data.data
      console.log(token.value)
      
      localStorage.setItem('token', token.value)
      setAuthHeader(token.value)
      
      return { success: true }
    } catch (error) {
      return { 
        success: false, 
        message: error.response?.data?.message || '登录失败' 
      }
    } finally {
      loading.value = false
    }
  }

  // 注册
  const register = async (userData) => {
    loading.value = true
    try {
      const response = await axios.post('/api/user/register', userData)
      return { success: true, message: '注册成功' }
    } catch (error) {
      return {
        success: false,
        message: error.response?.data?.message || '注册失败'
      }
    } finally {
      loading.value = false
    }
  }

  // 登出
  const logout = () => {
    token.value = null
    user.value = null
    localStorage.removeItem('token')
    setAuthHeader(null)
  }

  // 检查认证状态
  const checkAuth = async () => {
    if (!token.value) return
    
    try {
      setAuthHeader(token.value)
      const response = await axios.get('/api/auth/me')
      user.value = response.data
    } catch (error) {
      logout()
    }
  }

  // 更新用户信息
  const updateProfile = async (profileData) => {
    loading.value = true
    try {
      const response = await axios.put('/api/auth/profile', profileData)
      user.value = { ...user.value, ...response.data }
      return { success: true, message: '更新成功' }
    } catch (error) {
      return {
        success: false,
        message: error.response?.data?.message || '更新失败'
      }
    } finally {
      loading.value = false
    }
  }

  return {
    user,
    token,
    loading,
    isAuthenticated,
    login,
    register,
    logout,
    checkAuth,
    updateProfile
  }
})