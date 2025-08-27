<template>
  <div class="notifications-container">
    <!-- 通知中心头部 -->
    <div class="notifications-header">
      <h1 class="header-title">通知中心</h1>
      <div class="header-actions">
        <el-badge :value="unreadCount" class="notification-badge">
          <span>未读通知</span>
        </el-badge>
        <el-button class="header-btn secondary" @click="markAllRead">
          <i class="fas fa-check-double"></i>
          全部已读
        </el-button>
        <el-button type="primary" class="header-btn" @click="refreshNotifications">
          <i class="fas fa-sync"></i>
          刷新
        </el-button>
      </div>
    </div>
    
    <!-- 通知内容 -->
    <div class="notifications-content">
      <!-- 左侧筛选栏 -->
      <div class="notifications-sidebar">
        <div class="sidebar-section">
          <h3 class="sidebar-title">通知类型</h3>
          <div class="filter-list">
            <div 
              v-for="filter in notificationFilters" 
              :key="filter.key"
              :class="['filter-item', { active: selectedFilter === filter.key }]"
              @click="selectedFilter = filter.key"
            >
              <div class="filter-info">
                <i :class="filter.icon"></i>
                <span class="filter-text">{{ filter.name }}</span>
              </div>
              <div class="filter-count">{{ filter.count }}</div>
            </div>
          </div>
        </div>
        
        <div class="sidebar-section">
          <h3 class="sidebar-title">通知设置</h3>
          <div class="settings-list">
            <div class="setting-item">
              <div class="setting-info">
                <i class="fas fa-bell"></i>
                <span>桌面通知</span>
              </div>
              <el-switch v-model="notificationSettings.desktop" />
            </div>
            <div class="setting-item">
              <div class="setting-info">
                <i class="fas fa-volume-up"></i>
                <span>声音提醒</span>
              </div>
              <el-switch v-model="notificationSettings.sound" />
            </div>
            <div class="setting-item">
              <div class="setting-info">
                <i class="fas fa-envelope"></i>
                <span>邮件通知</span>
              </div>
              <el-switch v-model="notificationSettings.email" />
            </div>
          </div>
        </div>
      </div>
      
      <!-- 右侧通知列表 -->
      <div class="notifications-main">
        <div class="notification-list">
          <div 
            v-for="notification in filteredNotifications" 
            :key="notification.id"
            :class="['notification-item', { unread: !notification.read }]"
            @click="markAsRead(notification)"
          >
            <div class="notification-avatar">
              <el-avatar :size="40" :src="notification.avatar">
                {{ notification.sender.charAt(0) }}
              </el-avatar>
            </div>
            <div class="notification-content">
              <div class="notification-header">
                <div class="notification-sender">{{ notification.sender }}</div>
                <div class="notification-time">{{ notification.time }}</div>
              </div>
              <div class="notification-message">{{ notification.message }}</div>
              <div class="notification-type">
                <el-tag :type="notification.type" size="small">
                  {{ getNotificationTypeName(notification.type) }}
                </el-tag>
              </div>
            </div>
            <div class="notification-actions">
              <el-button 
                v-if="!notification.read" 
                type="primary" 
                size="small" 
                @click.stop="markAsRead(notification)"
              >
                标记已读
              </el-button>
              <el-button 
                size="small" 
                @click.stop="deleteNotification(notification)"
              >
                删除
              </el-button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, computed } from 'vue'
import { ElMessage } from 'element-plus'

// 响应式数据
const selectedFilter = ref('all')
const unreadCount = ref(5)

// 通知筛选器
const notificationFilters = [
  { key: 'all', name: '全部通知', icon: 'fas fa-inbox', count: 12 },
  { key: 'message', name: '消息通知', icon: 'fas fa-comment', count: 5 },
  { key: 'friend', name: '好友请求', icon: 'fas fa-user-plus', count: 2 },
  { key: 'group', name: '群组通知', icon: 'fas fa-users', count: 3 },
  { key: 'system', name: '系统通知', icon: 'fas fa-cog', count: 2 }
]

// 通知设置
const notificationSettings = reactive({
  desktop: true,
  sound: true,
  email: false
})

// 模拟通知数据
const notifications = reactive([
  {
    id: 1,
    sender: '张三',
    avatar: '',
    message: '发送了一条新消息给你',
    type: 'message',
    time: '5分钟前',
    read: false
  },
  {
    id: 2,
    sender: '李四',
    avatar: '',
    message: '想要添加你为好友',
    type: 'friend',
    time: '1小时前',
    read: false
  },
  {
    id: 3,
    sender: '前端开发群',
    avatar: '',
    message: '你被邀请加入群组',
    type: 'group',
    time: '2小时前',
    read: true
  },
  {
    id: 4,
    sender: '系统',
    avatar: '',
    message: '你的账号安全设置已更新',
    type: 'system',
    time: '昨天',
    read: true
  }
])

// 计算属性
const filteredNotifications = computed(() => {
  if (selectedFilter.value === 'all') {
    return notifications
  }
  return notifications.filter(n => n.type === selectedFilter.value)
})

// 方法
const getNotificationTypeName = (type) => {
  const typeMap = {
    message: '消息',
    friend: '好友',
    group: '群组',
    system: '系统'
  }
  return typeMap[type] || '其他'
}

const markAsRead = (notification) => {
  notification.read = true
  unreadCount.value = Math.max(0, unreadCount.value - 1)
}

const markAllRead = () => {
  notifications.forEach(n => n.read = true)
  unreadCount.value = 0
  ElMessage.success('所有通知已标记为已读')
}

const deleteNotification = (notification) => {
  const index = notifications.findIndex(n => n.id === notification.id)
  if (index > -1) {
    notifications.splice(index, 1)
    if (!notification.read) {
      unreadCount.value = Math.max(0, unreadCount.value - 1)
    }
    ElMessage.success('通知已删除')
  }
}

const refreshNotifications = () => {
  ElMessage.success('通知已刷新')
}
</script>

<style scoped>
.notifications-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
}

.notifications-header {
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 12px rgba(0,0,0,0.1);
  padding: 30px;
  margin-bottom: 20px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header-title {
  font-size: 28px;
  font-weight: 600;
  color: #333;
}

.header-actions {
  display: flex;
  gap: 15px;
  align-items: center;
}

.notification-badge {
  background: #f56c6c;
  color: white;
  border-radius: 10px;
  padding: 4px 8px;
  font-size: 12px;
  font-weight: 600;
}

.header-btn {
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  gap: 8px;
}

.header-btn.primary {
  background: #409EFF;
  color: white;
}

.header-btn.primary:hover {
  background: #66b1ff;
}

.header-btn.secondary {
  background: #f0f0f0;
  color: #666;
}

.header-btn.secondary:hover {
  background: #e0e0e0;
}

.notifications-content {
  display: grid;
  grid-template-columns: 300px 1fr;
  gap: 20px;
}

.notifications-sidebar {
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 12px rgba(0,0,0,0.1);
  padding: 20px;
  height: fit-content;
}

.sidebar-section {
  margin-bottom: 25px;
}

.sidebar-title {
  font-size: 16px;
  font-weight: 600;
  color: #333;
  margin-bottom: 15px;
  padding-bottom: 10px;
  border-bottom: 2px solid #f0f0f0;
}

.filter-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.filter-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 15px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s;
  border: 2px solid transparent;
}

.filter-item:hover {
  background: #f0f0f0;
}

.filter-item.active {
  background: #e6f7ff;
  border-color: #409EFF;
  color: #409EFF;
}

.filter-info {
  display: flex;
  align-items: center;
  gap: 10px;
}

.filter-icon {
  width: 20px;
  text-align: center;
  font-size: 16px;
}

.filter-text {
  font-size: 14px;
  font-weight: 500;
}

.filter-count {
  background: #409EFF;
  color: white;
  border-radius: 10px;
  padding: 2px 8px;
  font-size: 12px;
  font-weight: 600;
}

.settings-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 15px;
  border-radius: 8px;
  background: #fafafa;
  transition: all 0.3s;
}

.setting-item:hover {
  background: #f0f0f0;
}

.setting-info {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 14px;
}

.notifications-main {
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 12px rgba(0,0,0,0.1);
  padding: 20px;
}

.notification-list {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.notification-item {
  display: flex;
  gap: 15px;
  padding: 20px;
  border-radius: 12px;
  background: #fafafa;
  cursor: pointer;
  transition: all 0.3s;
  border: 2px solid transparent;
}

.notification-item:hover {
  background: #f0f0f0;
  transform: translateY(-2px);
}

.notification-item.unread {
  background: #e6f7ff;
  border-color: #409EFF;
}

.notification-avatar {
  flex-shrink: 0;
}

.notification-content {
  flex: 1;
}

.notification-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.notification-sender {
  font-weight: 600;
  color: #333;
}

.notification-time {
  font-size: 12px;
  color: #999;
}

.notification-message {
  color: #666;
  margin-bottom: 8px;
  line-height: 1.4;
}

.notification-type {
  margin-bottom: 8px;
}

.notification-actions {
  display: flex;
  flex-direction: column;
  gap: 8px;
  flex-shrink: 0;
}
</style>