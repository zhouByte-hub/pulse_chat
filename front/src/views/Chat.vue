<template>
  <DefaultLayout>
    <div class="chat-container">
      <!-- 左侧联系人列表 -->
      <div class="sidebar">
        <div class="search-box">
          <el-input
            v-model="searchQuery"
            placeholder="搜索联系人或消息..."
            prefix-icon="Search"
            clearable
            @input="handleSearch"
          />
        </div>
        
        <div class="sidebar-nav">
          <div class="nav-tabs">
            <div 
              class="nav-tab"
              :class="{ active: activeTab === 'messages' }"
              @click="activeTab = 'messages'"
            >
              <el-icon><Message /></el-icon>
              消息
            </div>
            <div 
              class="nav-tab"
              :class="{ active: activeTab === 'contacts' }"
              @click="activeTab = 'contacts'"
            >
              <el-icon><User /></el-icon>
              联系人
            </div>
            <div 
              class="nav-tab"
              :class="{ active: activeTab === 'groups' }"
              @click="activeTab = 'groups'"
            >
              <el-icon><UserFilled /></el-icon>
              群组
            </div>
          </div>
        </div>
        
        <div class="contact-list">
          <div 
            v-for="contact in filteredContacts" 
            :key="contact.id"
            :class="['contact-item', { active: selectedContact?.id === contact.id }]"
            @click="selectContact(contact)"
          >
            <el-avatar :size="40" :src="contact.avatar">
              {{ contact.name.charAt(0) }}
            </el-avatar>
            <div class="contact-info">
              <div class="contact-name">{{ contact.name }}</div>
              <div class="contact-message">{{ contact.lastMessage }}</div>
            </div>
            <div class="contact-meta">
              <div class="contact-time">{{ contact.time }}</div>
              <el-badge v-if="contact.unread" :value="contact.unread" class="contact-badge" />
            </div>
          </div>
        </div>
      </div>
      
      <!-- 右侧聊天区域 -->
      <div class="chat-area">
        <div v-if="selectedContact" class="chat-content">
          <!-- 聊天头部 -->
          <div class="chat-header">
            <el-avatar :size="40" :src="selectedContact.avatar">
              {{ selectedContact.name.charAt(0) }}
            </el-avatar>
            <div class="chat-user-info">
              <div class="chat-user-name">{{ selectedContact.name }}</div>
              <div class="chat-user-status">在线</div>
            </div>
            <div class="chat-actions">
              <el-button circle>
                <el-icon><Phone /></el-icon>
              </el-button>
              <el-button circle>
                <el-icon><VideoCamera /></el-icon>
              </el-button>
              <el-button circle>
                <el-icon><MoreFilled /></el-icon>
              </el-button>
            </div>
          </div>
          
          <!-- 消息区域 -->
          <div class="messages-container" ref="messagesContainer">
            <div 
              v-for="message in messages" 
              :key="message.id"
              :class="['message-item', { 'message-sent': message.sent }]"
            >
              <div class="message-content">
                <div class="message-text">{{ message.text }}</div>
                <div class="message-time">{{ message.time }}</div>
              </div>
            </div>
          </div>
          
          <!-- 输入区域 -->
          <div class="input-area">
            <el-button circle>
              <el-icon><Paperclip /></el-icon>
            </el-button>
            <el-input 
              v-model="newMessage" 
              placeholder="输入消息..."
              @keyup.enter="sendMessage"
            />
            <el-button circle>
              <el-icon><Star /></el-icon>
            </el-button>
            <el-button 
              type="primary" 
              circle
              @click="sendMessage"
            >
              <el-icon><Position /></el-icon>
            </el-button>
          </div>
        </div>
        
        <div v-else class="empty-chat">
          <el-icon :size="60" color="#C0C4CC"><ChatDotRound /></el-icon>
          <p>选择一个联系人开始聊天</p>
        </div>
      </div>
    </div>
  </DefaultLayout>
</template>

<script setup>
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { useChatStore } from '@/stores/chat'
import {
  ChatDotRound,
  User,
  Users,
  Clock,
  Bell,
  Phone,
  VideoCamera,
  MoreFilled,
  Paperclip,
  Star,
  Position,
  Message,
  UserFilled,
  Search
} from '@element-plus/icons-vue'

const router = useRouter()
const authStore = useAuthStore()
const chatStore = useChatStore()
const messagesContainer = ref(null)

// 响应式数据
const activeTab = ref('messages')
const selectedContact = ref(null)
const newMessage = ref('')
const notificationCount = ref(3)
const searchQuery = ref('')

// 用户信息
const userAvatar = computed(() => authStore.user?.avatar || '')
const userInitial = computed(() => authStore.user?.nickname?.charAt(0) || 'U')

// 模拟联系人数据
const contacts = reactive([
  {
    id: 1,
    name: '张三',
    avatar: '',
    lastMessage: '你好，最近怎么样？',
    time: '10:30',
    unread: 2
  },
  {
    id: 2,
    name: '李四',
    avatar: '',
    lastMessage: '明天见！',
    time: '昨天',
    unread: 0
  },
  {
    id: 3,
    name: '王五',
    avatar: '',
    lastMessage: '收到，谢谢',
    time: '周三',
    unread: 1
  }
])

// 模拟消息数据
const messages = reactive([
  { id: 1, text: '你好！', time: '10:25', sent: false },
  { id: 2, text: '你好，最近怎么样？', time: '10:26', sent: false },
  { id: 3, text: '挺好的，你呢？', time: '10:28', sent: true },
  { id: 4, text: '我也不错，最近在忙项目', time: '10:30', sent: false }
])

// 计算属性
const filteredContacts = computed(() => {
  if (!searchQuery.value) return contacts
  
  return contacts.filter(contact => 
    contact.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
    contact.lastMessage.toLowerCase().includes(searchQuery.value.toLowerCase())
  )
})

// 方法
const selectContact = (contact) => {
  selectedContact.value = contact
}

const sendMessage = () => {
  if (newMessage.value.trim() === '') return
  
  const message = {
    id: messages.length + 1,
    text: newMessage.value,
    time: new Date().toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' }),
    sent: true
  }
  
  messages.push(message)
  newMessage.value = ''
  
  // 滚动到底部
  nextTick(() => {
    if (messagesContainer.value) {
      messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
    }
  })
}

const goToProfile = () => {
  router.push('/profile')
}

const handleSearch = () => {
  // 处理搜索逻辑
}

onMounted(() => {
  // 初始化滚动到底部
  if (messagesContainer.value) {
    messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
  }
})
</script>

// 在聊天页面的样式部分添加以下代码
<style scoped>
.chat-container {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: linear-gradient(135deg, #f5f7fa 0%, #e4e7f1 100%);
}

.top-nav {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 20px;
  height: 60px;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  z-index: 10;
}

.tabber {
  display: flex;
  gap: 20px;
}

.tab-item {
  padding: 8px 16px;
  border-radius: 20px;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  gap: 8px;
}

.tab-item:hover {
  background: rgba(64, 158, 255, 0.1);
}

.tab-item.active {
  background: #409EFF;
  color: white;
}

.chat-main {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.sidebar {
  width: 300px;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(5px);
  border-right: 1px solid rgba(0, 0, 0, 0.05);
  display: flex;
  flex-direction: column;
  transition: all 0.3s ease;
}

.contact-item {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  cursor: pointer;
  transition: all 0.2s ease;
  border-radius: 10px;
  margin: 0 8px;
}

.contact-item:hover {
  background: rgba(64, 158, 255, 0.1);
}

.contact-item.active {
  background: rgba(64, 158, 255, 0.2);
}

.chat-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: rgba(255, 255, 255, 0.5);
  backdrop-filter: blur(5px);
}

.chat-header {
  display: flex;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid rgba(0, 0, 0, 0.05);
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(5px);
}

.messages-container {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
  background: url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxMDAlIiBoZWlnaHQ9IjEwMCUiPjxkZWZzPjxwYXR0ZXJuIGlkPSJncmlkIiB3aWR0aD0iNDAiIGhlaWdodD0iNDAiIHBhdHRlcm5Vbml0cz0idXNlclNwYWNlT25Vc2UiIHBhdHRlcm5UcmFuc2Zvcm09InJvdGF0ZSg0NSkiPjxjaXJjbGUgY3g9IjEwIiBjeT0iMTAiIHI9IjAuNSIgZmlsbD0icmdiYSgwLCAwLCAwLCAwLjEpIi8+PC9wYXR0ZXJuPjwvZGVmcz48cmVjdCB3aWR0aD0iMTAwJSIgaGVpZ2h0PSIxMDAlIiBmaWxsPSJ1cmwoI2dyaWQpIi8+PC9zdmc+') repeat;
}

.message-item {
  margin-bottom: 16px;
  display: flex;
}

.message-sent {
  justify-content: flex-end;
}

.message-content {
  max-width: 70%;
  padding: 12px 16px;
  border-radius: 18px;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
  animation: fadeIn 0.3s ease;
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

.input-area {
  display: flex;
  align-items: center;
  padding: 16px 20px;
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(5px);
  border-top: 1px solid rgba(0, 0, 0, 0.05);
  gap: 12px;
}

.input-area .el-input {
  flex: 1;
}

@media (max-width: 768px) {
  .sidebar {
    width: 80px;
  }
  
  .nav-tab span {
    display: none;
  }
  
  .contact-info {
    display: none;
  }
  
  .contact-meta {
    align-items: center;
  }
}
</style>