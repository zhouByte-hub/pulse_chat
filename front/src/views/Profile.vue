<template>
  <div class="profile-container">
    <!-- 个人资料头部 -->
    <div class="profile-header">
      <div class="profile-avatar">
        <el-avatar :size="120" :src="userInfo.avatar">
          {{ userInfo.username?.charAt(0)?.toUpperCase() }}
        </el-avatar>
        <div class="avatar-upload" @click="uploadAvatar">
          <i class="fas fa-camera"></i>
        </div>
      </div>
      <h2 class="profile-name">{{ userInfo.nickname || userInfo.username }}</h2>
      <p class="profile-username">@{{ userInfo.username }}</p>
      <div class="profile-status">
        <i class="fas fa-circle"></i>
        <span>在线</span>
      </div>
      <div class="profile-actions">
        <el-button type="primary" class="action-btn" @click="editProfile">
          <i class="fas fa-edit"></i>
          编辑资料
        </el-button>
        <el-button class="action-btn secondary" @click="shareProfile">
          <i class="fas fa-share"></i>
          分享
        </el-button>
      </div>
    </div>
    
    <!-- 个人资料内容 -->
    <div class="profile-content">
      <div class="profile-tabs">
        <div 
          v-for="tab in tabs" 
          :key="tab.key"
          :class="['tab-item', { active: activeTab === tab.key }]"
          @click="activeTab = tab.key"
        >
          {{ tab.name }}
        </div>
      </div>
      
      <div class="tab-content">
        <!-- 基本信息 -->
        <div v-if="activeTab === 'basic'" class="tab-pane active">
          <div class="info-section">
            <h3 class="section-title">基本信息</h3>
            <div class="info-grid">
              <div class="info-item">
                <div class="info-label">用户名</div>
                <div class="info-value">{{ userInfo.username }}</div>
              </div>
              <div class="info-item">
                <div class="info-label">昵称</div>
                <div class="info-value">{{ userInfo.nickname || '未设置' }}</div>
              </div>
              <div class="info-item">
                <div class="info-label">邮箱</div>
                <div class="info-value">{{ userInfo.email || '未设置' }}</div>
              </div>
              <div class="info-item">
                <div class="info-label">手机号</div>
                <div class="info-value">{{ userInfo.phone || '未设置' }}</div>
              </div>
              <div class="info-item">
                <div class="info-label">性别</div>
                <div class="info-value">{{ userInfo.gender || '未设置' }}</div>
              </div>
              <div class="info-item">
                <div class="info-label">生日</div>
                <div class="info-value">{{ userInfo.birthday || '未设置' }}</div>
              </div>
            </div>
          </div>
          
          <div class="info-section">
            <h3 class="section-title">个人简介</h3>
            <div class="bio-content">
              {{ userInfo.bio || '这个人很懒，什么都没有写...' }}
            </div>
          </div>
        </div>
        
        <!-- 账号设置 -->
        <div v-if="activeTab === 'account'" class="tab-pane active">
          <div class="info-section">
            <h3 class="section-title">安全设置</h3>
            <div class="security-list">
              <div class="security-item">
                <div class="security-info">
                  <div class="security-title">登录密码</div>
                  <div class="security-desc">定期更换密码，保护账号安全</div>
                </div>
                <el-button type="primary" size="small" @click="changePassword">
                  修改密码
                </el-button>
              </div>
              <div class="security-item">
                <div class="security-info">
                  <div class="security-title">绑定手机</div>
                  <div class="security-desc">用于账号安全验证和找回密码</div>
                </div>
                <el-button type="primary" size="small" @click="bindPhone">
                  {{ userInfo.phone ? '更换手机' : '绑定手机' }}
                </el-button>
              </div>
              <div class="security-item">
                <div class="security-info">
                  <div class="security-title">绑定邮箱</div>
                  <div class="security-desc">用于接收重要通知和验证</div>
                </div>
                <el-button type="primary" size="small" @click="bindEmail">
                  {{ userInfo.email ? '更换邮箱' : '绑定邮箱' }}
                </el-button>
              </div>
            </div>
          </div>
        </div>
        
        <!-- 隐私设置 -->
        <div v-if="activeTab === 'privacy'" class="tab-pane active">
          <div class="info-section">
            <h3 class="section-title">隐私设置</h3>
            <div class="privacy-list">
              <div class="privacy-item">
                <div class="privacy-info">
                  <div class="privacy-title">在线状态</div>
                  <div class="privacy-desc">控制其他人是否能看到你的在线状态</div>
                </div>
                <el-switch v-model="privacySettings.showOnline" />
              </div>
              <div class="privacy-item">
                <div class="privacy-info">
                  <div class="privacy-title">最后上线时间</div>
                  <div class="privacy-desc">控制其他人是否能看到你最后上线时间</div>
                </div>
                <el-switch v-model="privacySettings.showLastSeen" />
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { ElMessage } from 'element-plus'

const router = useRouter()
const authStore = useAuthStore()

// 响应式数据
const activeTab = ref('basic')

// 标签页配置
const tabs = [
  { key: 'basic', name: '基本信息' },
  { key: 'account', name: '账号设置' },
  { key: 'privacy', name: '隐私设置' }
]

// 隐私设置
const privacySettings = reactive({
  showOnline: true,
  showLastSeen: true,
  profileVisibility: 'contacts'
})

// 计算属性
const userInfo = computed(() => {
  return authStore.user || {
    username: '未登录用户',
    nickname: '',
    avatar: '',
    email: '',
    phone: '',
    gender: '',
    birthday: '',
    bio: ''
  }
})

// 方法
const uploadAvatar = () => {
  ElMessage.info('头像上传功能开发中')
}

const editProfile = () => {
  ElMessage.info('编辑资料功能开发中')
}

const shareProfile = () => {
  ElMessage.info('分享功能开发中')
}

const changePassword = () => {
  ElMessage.info('修改密码功能开发中')
}

const bindPhone = () => {
  ElMessage.info('绑定手机功能开发中')
}

const bindEmail = () => {
  ElMessage.info('绑定邮箱功能开发中')
}
</script>

<style scoped>
.profile-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}

.profile-header {
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
  padding: 30px;
  margin-bottom: 20px;
  text-align: center;
  position: relative;
}

.profile-avatar {
  width: 120px;
  height: 120px;
  border-radius: 50%;
  margin: 0 auto 20px;
  position: relative;
  overflow: hidden;
  border: 4px solid #409EFF;
}

.avatar-upload {
  position: absolute;
  bottom: 0;
  right: 0;
  background: #409EFF;
  color: white;
  width: 36px;
  height: 36px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  border: 3px solid white;
  transition: all 0.3s;
}

.avatar-upload:hover {
  background: #66b1ff;
  transform: scale(1.1);
}

.profile-name {
  font-size: 28px;
  font-weight: 600;
  color: #333;
  margin-bottom: 8px;
}

.profile-username {
  color: #666;
  font-size: 16px;
  margin-bottom: 15px;
}

.profile-status {
  display: inline-flex;
  align-items: center;
  background: #f0f9ff;
  color: #409EFF;
  padding: 6px 16px;
  border-radius: 20px;
  font-size: 14px;
  margin-bottom: 20px;
}

.profile-status i {
  margin-right: 6px;
  font-size: 8px;
}

.profile-actions {
  display: flex;
  gap: 15px;
  justify-content: center;
}

.action-btn {
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

.action-btn.secondary {
  background: #f0f0f0;
  color: #666;
}

.action-btn.secondary:hover {
  background: #e0e0e0;
}

.profile-content {
  background: white;
  border-radius: 12px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.profile-tabs {
  display: flex;
  border-bottom: 1px solid #e6e6e6;
  background: #fafafa;
}

.tab-item {
  flex: 1;
  padding: 15px 20px;
  text-align: center;
  cursor: pointer;
  transition: all 0.3s;
  border-bottom: 3px solid transparent;
  color: #666;
  font-size: 14px;
  font-weight: 500;
}

.tab-item:hover {
  background: #f0f0f0;
}

.tab-item.active {
  color: #409EFF;
  border-bottom-color: #409EFF;
  background: white;
}

.tab-content {
  padding: 30px;
}

.info-section {
  margin-bottom: 30px;
}

.section-title {
  font-size: 18px;
  font-weight: 600;
  color: #333;
  margin-bottom: 20px;
  padding-bottom: 10px;
  border-bottom: 2px solid #f0f0f0;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 20px;
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.info-label {
  font-size: 14px;
  color: #666;
  font-weight: 500;
}

.info-value {
  font-size: 16px;
  color: #333;
  font-weight: 500;
}

.bio-content {
  line-height: 1.6;
  color: #666;
  font-size: 14px;
}

.security-list,
.privacy-list {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.security-item,
.privacy-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px;
  background: #fafafa;
  border-radius: 8px;
  transition: all 0.3s;
}

.security-item:hover,
.privacy-item:hover {
  background: #f0f0f0;
}

.security-info,
.privacy-info {
  flex: 1;
}

.security-title,
.privacy-title {
  font-size: 16px;
  font-weight: 500;
  color: #333;
  margin-bottom: 4px;
}

.security-desc,
.privacy-desc {
  font-size: 14px;
  color: #666;
}
</style>