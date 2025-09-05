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

    return {
        searchContact
    }
})