import { defineStore } from 'pinia'
import { ref } from 'vue'
import axios from 'axios'

export const useChatStore = defineStore('chat', () => {

  const token = ref(localStorage.getItem('token') || null)

  const updateMessageStatus = async (user_id) => {
        try {
            axios.defaults.headers.common['Authorization'] = `Bearer ${token.value}`
            await axios.get(`/api/message/update_status/${user_id}`)
            return {
                success: true,
                message: '更新消息状态成功'
            }
        } catch (error) {
            return {
                success: false,
                message: error.response?.data?.message || '更新消息状态失败'
            }
        }
    }

    const sendMessage = async (message) => {
        try {
            axios.defaults.headers.common['Authorization'] = `Bearer ${token.value}`
            await axios.post('/api/message/send', message)
            return {
                success: true,
                message: '发送消息成功'
            }
        } catch (error) {
            return {
                success: false,
                message: error.response?.data?.message || '发送消息失败'
            }
        }
    }

    const historyMessage = async (receive_id) => {
        try {
            axios.defaults.headers.common['Authorization'] = `Bearer ${token.value}`
            let response = await axios.get(`/api/message/history/${receive_id}`)
            return {
                success: true,
                data: response.data.data
            }
        } catch (error) {
            return {
                success: false,
                message: error.response?.data?.message || '获取对话记录失败'
            }
        }
    }


  return {
    updateMessageStatus,
    sendMessage,
    historyMessage
  }
})