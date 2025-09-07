import { defineStore } from 'pinia'
import axios from 'axios'

export const useResetPasswordStore = defineStore('resetPassword', () => {


    const sendForgetPasswordEmail = async (email) => {
        try {
            await axios.post('/api/user/send_forget_password_email', { email })
            return { success: true, message: '邮件发送成功' }
        } catch (error) {
            return {
                success: false,
                message: error.response?.data?.message || '邮件发送失败'
            }
        }
    }

    const resetPassword = async (request) => {
        try {
            await axios.post('/api/user/reset_password', request)
            return { success: true, message: '密码重置成功' }
        } catch (error) {
            return {
                success: false,
                message: error.response?.data?.message || '密码重置失败'
            }
        }
    }

    return {
        sendForgetPasswordEmail,
        resetPassword
    }

})