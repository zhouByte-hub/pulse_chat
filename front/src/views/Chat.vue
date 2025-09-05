<template>
  <DefaultLayout>
    <div class="chat-container">
      <!-- 左侧边栏 -->
      <div class="sidebar">
        <!-- 侧边栏头部 -->
        <div class="sidebar-header">
          <div class="user-info">
            <el-avatar :size="36" class="user-avatar">
              {{ userInitial }}
            </el-avatar>
            <div class="user-details">
              <div class="user-name">{{ userInfo.username || '用户' }}</div>
              <div class="user-status">在线</div>
            </div>
          </div>
          <div class="header-actions">
            <el-button size="small" circle @click="goToProfile">
              <el-icon><Setting /></el-icon>
            </el-button>
          </div>
        </div>

        <!-- 搜索框 -->
        <div class="search-container">
          <el-input
            v-model="searchQuery"
            placeholder="搜索联系人..."
            prefix-icon="Search"
            clearable
            @keyup.enter="handleSearch"
            class="search-input"/>
        </div>

        <!-- 标签导航 -->
        <div class="nav-tabs">
          <div 
            class="nav-tab"
            :class="{ active: activeTab === 'messages' }"
            @click="activeTab = 'messages'"
          >
            <el-icon><ChatDotRound /></el-icon>
            <span>消息</span>
            <el-badge v-if="totalUnread > 0" :value="totalUnread" class="tab-badge" />
          </div>
          <div 
            class="nav-tab"
            :class="{ active: activeTab === 'contacts' }"
            @click="activeTab = 'contacts'"
          >
            <el-icon><User /></el-icon>
            <span>联系人</span>
          </div>
          <div 
            class="nav-tab"
            :class="{ active: activeTab === 'groups' }"
            @click="activeTab = 'groups'"
          >
            <el-icon><UserFilled /></el-icon>
            <span>群组</span>
          </div>
        </div>

        <!-- 联系人列表 -->
        <div class="contact-list">
          <el-scrollbar height="100%">
            <div 
              v-for="contact in filteredContacts" 
              :key="contact.id"
              :class="['contact-item', { active: selectedContact?.id === contact.id }]"
              @click="selectContact(contact)"
            >
              <div class="contact-avatar-wrapper">
                <el-avatar :size="44" :src="contact.avatar" class="contact-avatar">
                  {{ contact.username.charAt(0).toUpperCase() }}
                </el-avatar>
                <div v-if="contact.status == 'online'" class="online-indicator"></div>
              </div>
              <div class="contact-info">
                <div class="contact-header">
                  <span class="contact-name">{{ contact.username }}</span>
                  <span class="contact-time">{{ new Date(new Date().getTime()).toLocaleString() }}</span>
                </div>
                <div class="contact-message">{{ contact.last_message }}</div>
              </div>
              <div class="contact-badge-wrapper">
                <el-badge v-if="contact.unread_count != 0" :value="contact.unread_count" />
              </div>
            </div>
          </el-scrollbar>
        </div>
      </div>

      <!-- 右侧聊天区域 -->
      <div class="chat-area">
        <div v-if="selectedContact" class="chat-content">
          <!-- 聊天头部 -->
          <div class="chat-header">
            <div class="chat-user-info">
              <el-avatar :size="40" :src="selectedContact.avatar" class="chat-avatar">
                {{ selectedContact.username.charAt(0) }}
              </el-avatar>
              <div class="chat-user-details">
                <div class="chat-user-name">{{ selectedContact.username }}</div>
                <div class="chat-user-status">
                  <div class="status-dot" :class="{ online: selectedContact.status == 'online' }"></div>
                  {{ selectedContact.status == 'online' ? '在线' : '离线' }}
                </div>
              </div>
            </div>
            <div class="chat-actions">
              <el-button circle class="action-btn">
                <el-icon><Phone /></el-icon>
              </el-button>
              <el-button circle class="action-btn">
                <el-icon><VideoCamera /></el-icon>
              </el-button>
              <el-button circle class="action-btn">
                <el-icon><MoreFilled /></el-icon>
              </el-button>
            </div>
          </div>

          <!-- 消息区域 -->
          <div class="messages-container" ref="messagesContainer">
            <el-scrollbar height="100%">
              <div class="messages-wrapper">
                <div 
                  v-for="message in currentMessages" 
                  :key="message.id"
                  :class="['message-group', { 'message-sent': message.sent }]"
                >
                  <div v-if="!message.sent" class="message-avatar">
                    <el-avatar :size="32" :src="selectedContact.avatar">
                      {{ selectedContact.name.charAt(0) }}
                    </el-avatar>
                  </div>
                  <div class="message-content">
                    <div class="message-bubble">
                      <div class="message-text">{{ message.text }}</div>
                    </div>
                    <div class="message-time">{{ message.time }}</div>
                  </div>
                  <div v-if="message.sent" class="message-avatar message-avatar-sent">
                    <el-avatar :size="32">
                      {{ userInitial }}
                    </el-avatar>
                  </div>
                </div>
              </div>
            </el-scrollbar>
          </div>

          <!-- 输入区域 -->
          <div class="input-area">
            <div class="input-toolbar">
              <el-button circle class="toolbar-btn">
                <el-icon><Paperclip /></el-icon>
              </el-button>
              <el-button circle class="toolbar-btn">
                <el-icon><Picture /></el-icon>
              </el-button>
              <el-button circle class="toolbar-btn">
                <el-icon><Microphone /></el-icon>
              </el-button>
            </div>
            <div class="input-wrapper">
              <el-input 
                v-model="newMessage" 
                placeholder="输入消息..."
                type="textarea"
                :rows="1"
                :autosize="{ minRows: 1, maxRows: 4 }"
                resize="none"
                @keyup.enter.exact="sendMessage"
                @keyup.enter.shift.prevent
                class="message-input"
              />
              <el-button 
                type="primary" 
                circle
                @click="sendMessage"
                :disabled="!newMessage.trim()"
                class="send-btn"
              >
                <el-icon><Promotion /></el-icon>
              </el-button>
            </div>
          </div>
        </div>

        <!-- 空状态 -->
        <div v-else class="empty-chat">
          <div class="empty-content">
            <el-icon :size="80" color="#C0C4CC"><ChatDotRound /></el-icon>
            <h3>欢迎来到 Pulse Chat</h3>
            <p>选择一个联系人开始聊天</p>
          </div>
        </div>
      </div>

      <!-- <div>
        <el-popconfirm
          width="220"
          confirm-button-text="OK"
          cancel-button-text="No, Thanks"
        >
          <template #reference>
            <el-button>Delete</el-button>
          </template>
        </el-popconfirm>
      </div> -->
    </div>
  </DefaultLayout>
</template>

<script setup>
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import { useUserStore } from '@/stores/user'
import { useChatStore } from '@/stores/chat'
import DefaultLayout from '@/layouts/DefaultLayout.vue'

const router = useRouter()
const messagesContainer = ref(null)
const userStore = useUserStore()
const chatStore = useChatStore()
import { ElMessage } from 'element-plus'

// 响应式数据
const activeTab = ref('messages')
const selectedContact = ref(null)
const newMessage = ref('')
const searchQuery = ref('')
const filteredContacts = ref([])

// 用户信息
const userInfo = JSON.parse(localStorage.getItem("user"))
let userInitial = userInfo.username.charAt(0)
if (userInitial.match(/[a-zA-Z]/)) {
  userInitial = userInitial.toUpperCase()
}

// 模拟联系人数据
const contacts = reactive([])

// 消息数据映射
const messagesMap = reactive({})

const totalUnread = computed(() => {
  return contacts.reduce((sum, contact) => sum + contact.unread, 0)
})

const currentMessages = computed(() => {
  if (!selectedContact.value) return []
  return messagesMap[selectedContact.value.id] || []
})

// 方法
const selectContact = (contact) => {
  selectedContact.value = contact
  contact.unread = 0
  chatStore.updateMessageStatus(contact.id).then(res => {
    console.log(res.data)
  })
  // 滚动到底部
  nextTick(() => {
    scrollToBottom()
  })
}

const sendMessage = () => {
  if (!newMessage.value.trim() || !selectedContact.value) return
  
  const message = {
    id: Date.now(),
    text: newMessage.value.trim(),
    time: new Date().toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' }),
    sent: true
  }
  
  // 添加消息到对应联系人
  if (!messagesMap[selectedContact.value.id]) {
    messagesMap[selectedContact.value.id] = []
  }
  messagesMap[selectedContact.value.id].push(message)
  
  // 更新联系人最后消息
  const contact = contacts.find(c => c.id === selectedContact.value.id)
  if (contact) {
    contact.lastMessage = message.text
    contact.time = message.time
  }

  chatStore.sendMessage({
    receive_id: selectedContact.value.id,
    content: message.text
  }).then(res => {
    if(res.success) {
      ElMessage.success(res.message)
    }else{
      ElMessage.error(res.message)
    }
  })

  
  newMessage.value = ''
  
  // 滚动到底部
  nextTick(() => {
    scrollToBottom()
  })
}

const scrollToBottom = () => {
  nextTick(() => {
    const container = messagesContainer.value
    if (container && container.$el) {
      const scrollWrapper = container.$el.querySelector('.el-scrollbar__wrap')
      if (scrollWrapper) {
        scrollWrapper.scrollTop = scrollWrapper.scrollHeight
      }
    }
  })
}

const goToProfile = () => {
  router.push('/profile')
}

const handleSearch = () => {
  userStore.searchContact(searchQuery.value).then(res => {
      if (res.success) {
        for(let i of res.data) {
          let flag = false
          for(let j of filteredContacts.value) {
            if(j.id == i.id) {
              flag = true; break;
            }
          }
          if(!flag) filteredContacts.value.push(i)
        }
      }else{
        ElMessage.error(res.msg)
      }
  })
}

onMounted(() => {
  // 默认选择第一个联系人
  if (contacts.length > 0) {
    selectContact(contacts[0])
  }
})
</script>

<style scoped>
.chat-container {
  height: 100vh;
  display: flex;
  background: #f5f5f5;
  overflow: hidden;
}

/* 左侧边栏 */
.sidebar {
  width: 320px;
  background: #ffffff;
  border-right: 1px solid #e4e7ed;
  display: flex;
  flex-direction: column;
  box-shadow: 2px 0 10px rgba(0, 0, 0, 0.1);
}

.sidebar-header {
  padding: 20px 16px 16px;
  border-bottom: 1px solid #e4e7ed;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.user-avatar {
  background: #409EFF;
}

.user-details {
  display: flex;
  flex-direction: column;
}

.user-name {
  font-weight: 600;
  font-size: 16px;
  color: #303133;
}

.user-status {
  font-size: 12px;
  color: #67C23A;
}

.header-actions .el-button {
  color: #606266;
}

.search-container {
  padding: 16px;
  border-bottom: 1px solid #e4e7ed;
}

.search-input {
  --el-input-border-radius: 20px;
}

.nav-tabs {
  display: flex;
  padding: 16px;
  gap: 8px;
  border-bottom: 1px solid #e4e7ed;
}

.nav-tab {
  flex: 1;
  padding: 12px 8px;
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  font-size: 14px;
  color: #606266;
  position: relative;
}

.nav-tab:hover {
  background: #ecf5ff;
  color: #409EFF;
}

.nav-tab.active {
  background: #409EFF;
  color: white;
  box-shadow: 0 2px 8px rgba(64, 158, 255, 0.3);
}

.tab-badge {
  position: absolute;
  top: 4px;
  right: 8px;
}

.contact-list {
  flex: 1;
  overflow: hidden;
}

.contact-item {
  display: flex;
  align-items: center;
  padding: 16px;
  cursor: pointer;
  transition: all 0.2s ease;
  position: relative;
}

.contact-item::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  width: 4px;
  height: 100%;
  background: #409EFF;
  transform: scaleY(0);
  transition: transform 0.2s ease;
}

.contact-item:hover {
  background: #f5f7fa;
}

.contact-item.active {
  background: #ecf5ff;
}

.contact-item.active::before {
  transform: scaleY(1);
}

.contact-avatar-wrapper {
  position: relative;
  margin-right: 12px;
}

.contact-avatar {
  background: #409EFF;
}

.online-indicator {
  position: absolute;
  bottom: 2px;
  right: 2px;
  width: 12px;
  height: 12px;
  background: #67C23A;
  border: 2px solid white;
  border-radius: 50%;
}

.contact-info {
  flex: 1;
  min-width: 0;
}

.contact-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 4px;
}

.contact-name {
  font-weight: 600;
  font-size: 16px;
  color: #303133;
}

.contact-time {
  font-size: 12px;
  color: #909399;
}

.contact-message {
  font-size: 14px;
  color: #606266;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.contact-badge-wrapper {
  margin-left: 8px;
}

/* 右侧聊天区域 */
.chat-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: #ffffff;
}

.chat-content {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.chat-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px 24px;
  border-bottom: 1px solid #e4e7ed;
  background: #ffffff;
}

.chat-user-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.chat-avatar {
  background: #409EFF;
}

.chat-user-details {
  display: flex;
  flex-direction: column;
}

.chat-user-name {
  font-weight: 600;
  font-size: 18px;
  color: #303133;
}

.chat-user-status {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 14px;
  color: #606266;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #C0C4CC;
}

.status-dot.online {
  background: #67C23A;
}

.chat-actions {
  display: flex;
  gap: 12px;
}

.action-btn {
  color: #606266;
  border: 1px solid #dcdfe6;
}

.action-btn:hover {
  background: #ecf5ff;
  color: #409EFF;
}

.messages-container {
  flex: 1;
  padding: 0;
  background: #f8f9fa;
}

.messages-wrapper {
  padding: 20px 24px;
  min-height: 100%;
}

.message-group {
  display: flex;
  align-items: flex-start;
  margin-bottom: 16px;
  gap: 12px;
}

.message-sent {
  justify-content: flex-end;
}

.message-avatar {
  margin-top: 4px;
}

.message-avatar-sent {
  margin-top: 4px;
  order: 3;
}

.message-content {
  max-width: 70%;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.message-sent .message-content {
  align-items: flex-end;
  order: 2;
}

.message-bubble {
  padding: 12px 16px;
  border-radius: 18px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  position: relative;
  animation: messageSlideIn 0.3s ease;
}

@keyframes messageSlideIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.message-group:not(.message-sent) .message-bubble {
  background: white;
  color: #303133;
}

.message-sent .message-bubble {
  background: #409EFF;
  color: white;
}

.message-text {
  line-height: 1.5;
  word-wrap: break-word;
}

.message-time {
  font-size: 12px;
  color: #909399;
  margin-top: 4px;
}

.input-area {
  padding: 20px 24px;
  background: #ffffff;
  border-top: 1px solid #e4e7ed;
}

.input-toolbar {
  display: flex;
  gap: 12px;
  margin-bottom: 12px;
}

.toolbar-btn {
  color: #606266;
  border: 1px solid #dcdfe6;
}

.toolbar-btn:hover {
  background: #ecf5ff;
  color: #409EFF;
}

.input-wrapper {
  display: flex;
  align-items: flex-end;
  gap: 12px;
}

.message-input {
  flex: 1;
}

.message-input :deep(.el-textarea__inner) {
  border-radius: 10px;
  border: 1px solid #dcdfe6;
  padding: 12px 16px;
  resize: none;
}

.send-btn {
  background: #409EFF;
  border: none;
  margin-bottom: 10px;
  box-shadow: 0 2px 8px rgba(64, 158, 255, 0.3);
}

.send-btn:hover {
  background: #337ecc;
  box-shadow: 0 4px 12px rgba(64, 158, 255, 0.4);
}

.send-btn:disabled {
  background: #C0C4CC;
  box-shadow: none;
}

.empty-chat {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.empty-content {
  text-align: center;
  color: #909399;
}

.empty-content h3 {
  margin: 20px 0 8px;
  color: #606266;
}

.empty-content p {
  margin: 0;
  font-size: 14px;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .sidebar {
    width: 280px;
  }
  
  .chat-container {
    height: 100vh;
  }
  
  .nav-tab span {
    display: none;
  }
  
  .messages-wrapper {
    padding: 16px;
  }
  
  .input-area {
    padding: 16px;
  }
}

@media (max-width: 480px) {
  .sidebar {
    width: 100%;
    position: absolute;
    z-index: 100;
  }
  
  .chat-area {
    display: none;
  }
  
  .sidebar.collapsed .chat-area {
    display: flex;
  }
}

.profile-actions {
  display: flex;
  justify-content: center;
  gap: 12px;
}

.profile-action-btn {
  color: #606266;
  border: 1px solid #dcdfe6;
  width: 40px;
  height: 40px;
}

.profile-action-btn:hover {
  background: #ecf5ff;
  color: #409EFF;
}

.info-tabs {
  display: flex;
  border-bottom: 1px solid #e4e7ed;
  background: #fafafa;
}

.info-tab {
  flex: 1;
  padding: 12px 16px;
  text-align: center;
  cursor: pointer;
  transition: all 0.3s ease;
  font-size: 14px;
  color: #606266;
  border-bottom: 2px solid transparent;
}

.info-tab:hover {
  background: #f5f7fa;
  color: #409EFF;
}

.info-tab.active {
  color: #409EFF;
  background: #ffffff;
  border-bottom-color: #409EFF;
  font-weight: 600;
}

.info-content {
  flex: 1;
  overflow: hidden;
}

.info-section {
  padding: 20px;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  color: #303133;
  margin-bottom: 16px;
  padding-bottom: 8px;
  border-bottom: 1px solid #f0f0f0;
}

.contact-detail {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 0;
  border-bottom: 1px solid #f5f7fa;
  color: #606266;
}

.contact-detail .el-icon {
  color: #909399;
  font-size: 16px;
}

.profile-detail {
  color: #606266;
  line-height: 1.6;
  margin-bottom: 24px;
  padding: 16px;
  background: #f8f9fa;
  border-radius: 8px;
  font-size: 14px;
}

.shared-media-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 8px;
  margin-bottom: 24px;
}

.media-item {
  aspect-ratio: 1;
  border-radius: 8px;
  overflow: hidden;
  cursor: pointer;
  transition: transform 0.2s ease;
}

.media-item:hover {
  transform: scale(1.05);
}

.media-item img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.media-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
}

.media-item.large {
  aspect-ratio: 1;
  border-radius: 12px;
  overflow: hidden;
  cursor: pointer;
  transition: transform 0.2s ease;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.media-item.large:hover {
  transform: scale(1.02);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
}

.media-item.large img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.file-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  border: 1px solid #e4e7ed;
  border-radius: 8px;
  transition: all 0.2s ease;
  cursor: pointer;
}

.file-item:hover {
  background: #f5f7fa;
  border-color: #409EFF;
}

.file-icon {
  width: 40px;
  height: 40px;
  background: #ecf5ff;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #409EFF;
  font-size: 20px;
}

.file-info {
  flex: 1;
  min-width: 0;
}

.file-name {
  font-weight: 600;
  color: #303133;
  margin-bottom: 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.file-meta {
  font-size: 12px;
  color: #909399;
}

/* 响应式设计 */
@media (max-width: 1200px) {
  .user-info-panel {
    width: 260px;
  }
  
  .shared-media-grid {
    grid-template-columns: repeat(2, 1fr);
  }
  
  .media-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 768px) {
  .sidebar {
    width: 280px;
  }
  
  .user-info-panel {
    display: none;
  }
  
  .chat-container {
    height: 100vh;
  }
  
  .nav-tab span {
    display: none;
  }
  
  .messages-wrapper {
    padding: 16px;
  }
  
  .input-area {
    padding: 16px;
  }
}

@media (max-width: 480px) {
  .sidebar {
    width: 100%;
    position: absolute;
    z-index: 100;
  }
  
  .chat-area {
    display: none;
  }
  
  .sidebar.collapsed .chat-area {
    display: flex;
  }
}
</style>