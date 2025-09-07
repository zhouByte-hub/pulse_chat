import axios from 'axios'
import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useUserStore = defineStore("user", () => {
    const token = ref(localStorage.getItem('token') || null)

    const searchContact = async (contactName) => {
        try {
            axios.defaults.headers.common['Authorization'] = `Bearer ${token.value}`
            let response = await axios.get('/api/user/search_contact', {
                params: {
                    content: contactName
                }
            })
            return {
                success: true,
                data: response.data.data
            }
        } catch (error) {
            return {
                success: false,
                message: error.response?.data?.message || '搜索联系人失败'
            }
        }
    }


    const contactList = async () => {
         try {
            axios.defaults.headers.common['Authorization'] = `Bearer ${token.value}`
            let response = await axios.get('/api/user/contact_list')
            return {
                success: true,
                data: response.data.data
            }
        } catch (error) {
            return {
                success: false,
                message: error.response?.data?.message || '获取联系人列表失败'
            }
        }
    }

    const sendResetEmail = async (email) => {
        try {
            let response = await axios.post('/api/user/send_reset_email', {
                email: email
            })
            return {
                success: true,
                message: response.data.message
            }
        } catch (error) {
            return {
                success: false,
                message: error.response?.data?.message || '发送重置邮件失败'
            }
        }
    }

    return {
        searchContact,
        contactList
    }
})