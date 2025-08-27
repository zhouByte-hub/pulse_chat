<template>
  <div class="settings-container">
    <!-- 左侧菜单 -->
    <div class="settings-sidebar">
      <div class="sidebar-header">
        <div class="sidebar-title">设置</div>
      </div>
      
      <div class="menu-list">
        <div 
          v-for="item in menuItems" 
          :key="item.key"
          class="menu-item" 
          :class="{ active: activeMenu === item.key }"
          @click="switchMenu(item.key)"
        >
          <el-icon class="menu-icon">
            <component :is="item.icon" />
          </el-icon>
          {{ item.label }}
        </div>
      </div>
    </div>
    
    <!-- 右侧内容区域 -->
    <div class="settings-content">
      <!-- 个人信息页面 -->
      <div v-if="activeMenu === 'profile'" class="profile-content">
        <div class="content-header">
          <div class="content-title">个人信息</div>
        </div>
        
        <div class="content-body">
          <div class="profile-section">
            <div class="profile-header">
              <div class="profile-avatar">
                {{ userStore.user?.username?.charAt(0).toUpperCase() || 'U' }}
                <div class="avatar-upload" @click="uploadAvatar">
                  <el-icon><Camera /></el-icon>
                </div>
              </div>
              <div class="profile-info">
                <h3>{{ userStore.user?.username || '未设置用户名' }}</h3>
                <p>ID: {{ userStore.user?.id || '未知' }}</p>
              </div>
            </div>
          </div>
          
          <el-form 
            :model="profileForm" 
            :rules="profileRules" 
            ref="profileFormRef"
            label-width="120px"
          >
            <div class="form-section">
              <div class="section-title">基本信息</div>
              
              <el-row :gutter="20">
                <el-col :span="12">
                  <el-form-item label="用户名" prop="username">
                    <el-input v-model="profileForm.username" />
                  </el-form-item>
                </el-col>
                <el-col :span="12">
                  <el-form-item label="昵称" prop="nickname">
                    <el-input v-model="profileForm.nickname" />
                  </el-form-item>
                </el-col>
              </el-row>
              
              <el-row :gutter="20">
                <el-col :span="12">
                  <el-form-item label="邮箱" prop="email">
                    <el-input v-model="profileForm.email" />
                  </el-form-item>
                </el-col>
                <el-col :span="12">
                  <el-form-item label="手机号" prop="phone">
                    <el-input v-model="profileForm.phone" />
                  </el-form-item>
                </el-col>
              </el-row>
              
              <el-form-item label="个人简介" prop="bio">
                <el-input 
                  v-model="profileForm.bio" 
                  type="textarea" 
                  :rows="4"
                  placeholder="介绍一下你自己..."
                />
              </el-form-item>
            </div>
            
            <div class="form-section">
              <el-button type="primary" @click="saveProfile">保存修改</el-button>
              <el-button @click="resetProfile">重置</el-button>
            </div>
          </el-form>
        </div>
      </div>
      
      <!-- 账号设置页面 -->
      <div v-if="activeMenu === 'account'" class="account-content">
        <div class="content-header">
          <div class="content-title">账号设置</div>
        </div>
        
        <div class="content-body">
          <el-form 
            :model="accountForm" 
            :rules="accountRules" 
            ref="accountFormRef"
            label-width="120px"
          >
            <div class="form-section">
              <div class="section-title">密码设置</div>
              
              <el-form-item label="当前密码" prop="currentPassword">
                <el-input 
                  v-model="accountForm.currentPassword" 
                  type="password" 
                  placeholder="请输入当前密码"
                  show-password
                />
              </el-form-item>
              
              <el-form-item label="新密码" prop="newPassword">
                <el-input 
                  v-model="accountForm.newPassword" 
                  type="password" 
                  placeholder="请输入新密码"
                  show-password
                />
              </el-form-item>
              
              <el-form-item label="确认新密码" prop="confirmPassword">
                <el-input 
                  v-model="accountForm.confirmPassword" 
                  type="password" 
                  placeholder="请再次输入新密码"
                  show-password
                />
              </el-form-item>
              
              <el-button type="primary" @click="changePassword">修改密码</el-button>
            </div>
            
            <div class="form-section">
              <div class="section-title">两步验证</div>
              
              <div class="switch-group">
                <div class="switch-info">
                  <div class="switch-label">启用两步验证</div>
                  <div class="switch-description">为您的账号添加额外的安全保护</div>
                </div>
                <el-switch v-model="accountForm.twoFactorEnabled" />
              </div>
              
              <div v-if="accountForm.twoFactorEnabled" class="two-factor-setup">
                <el-alert 
                  title="两步验证设置" 
                  type="info" 
                  :closable="false"
                  description="请使用身份验证器应用扫描下方二维码"
                />
                <div class="qr-code-placeholder">
                  <el-icon><QrCode /></el-icon>
                  <span>二维码占位符</span>
                </div>
              </div>
            </div>
          </el-form>
        </div>
      </div>
      
      <!-- 隐私设置页面 -->
      <div v-if="activeMenu === 'privacy'" class="privacy-content">
        <div class="content-header">
          <div class="content-title">隐私设置</div>
        </div>
        
        <div class="content-body">
          <div class="form-section">
            <div class="section-title">个人资料可见性</div>
            
            <div class="switch-group">
              <div class="switch-info">
                <div class="switch-label">显示在线状态</div>
                <div class="switch-description">让其他用户看到您是否在线</div>
              </div>
              <el-switch v-model="privacyForm.showOnlineStatus" />
            </div>
            
            <div class="switch-group">
              <div class="switch-info">
                <div class="switch-label">显示最后上线时间</div>
                <div class="switch-description">让其他用户看到您最后上线的时间</div>
              </div>
              <el-switch v-model="privacyForm.showLastSeen" />
            </div>
            
            <div class="switch-group">
              <div class="switch-info">
                <div class="switch-label">显示个人资料</div>
                <div class="switch-description">让其他用户可以查看您的个人资料</div>
              </div>
              <el-switch v-model="privacyForm.showProfile" />
            </div>
          </div>
          
          <div class="form-section">
            <div class="section-title">消息隐私</div>
            
            <div class="switch-group">
              <div class="switch-info">
                <div class="switch-label">接收来自陌生人的消息</div>
                <div class="switch-description">允许非联系人向您发送消息</div>
              </div>
              <el-switch v-model="privacyForm.allowStrangerMessages" />
            </div>
            
            <div class="switch-group">
              <div class="switch-info">
                <div class="switch-label">消息已读回执</div>
                <div class="switch-description">让对方知道您已阅读消息</div>
              </div>
              <el-switch v-model="privacyForm.sendReadReceipts" />
            </div>
          </div>
          
          <div class="form-section">
            <el-button type="primary" @click="savePrivacySettings">保存设置</el-button>
          </div>
        </div>
      </div>
      
      <!-- 通知设置页面 -->
      <div v-if="activeMenu === 'notification'" class="notification-content">
        <div class="content-header">
          <div class="content-title">通知设置</div>
        </div>
        
        <div class="content-body">
          <div class="form-section">
            <div class="section-title">桌面通知</div>
            
            <div class="switch-group">
              <div class="switch-info">
                <div class="switch-label">启用桌面通知</div>
                <div class="switch-description">在桌面上显示通知</div>
              </div>
              <el-switch v-model="notificationForm.enableDesktop" />
            </div>
            
            <div class="switch-group">
              <div class="switch-info">
                <div class="switch-label">消息通知</div>
                <div class="switch-description">新消息时显示通知</div>
              </div>
              <el-switch v-model="notificationForm.messageNotifications" />
            </div>
            
            <div class="switch-group">
              <div class="switch-info">
                <div class="switch-label">群组消息通知</div>
                <div class="switch-description">群组新消息时显示通知</div>
              </div>
              <el-switch v-model="notificationForm.groupNotifications" />
            </div>
            
            <div class="switch-group">
              <div class="switch-info">
                <div class="switch-label">好友请求通知</div>
                <div class="switch-description">收到好友请求时显示通知</div>
              </div>
              <el-switch v-model="notificationForm.friendRequestNotifications" />
            </div>
          </div>
          
          <div class="form-section">
            <div class="section-title">声音设置</div>
            
            <div class="switch-group">
              <div class="switch-info">
                <div class="switch-label">消息提示音</div>
                <div class="switch-description">新消息时播放提示音</div>
              </div>
              <el-switch v-model="notificationForm.messageSound" />
            </div>
            
            <div class="switch-group">
              <div class="switch-info">
                <div class="switch-label">通知音量</div>
                <div class="switch-description">调整通知音量大小</div>
              </div>
              <el-slider 
                v-model="notificationForm.volume" 
                :min="0" 
                :max="100" 
                :disabled="!notificationForm.messageSound"
              />
            </div>
          </div>
          
          <div class="form-section">
            <el-button type="primary" @click="saveNotificationSettings">保存设置</el-button>
          </div>
        </div>
      </div>
      
      <!-- 外观设置页面 -->
      <div v-if="activeMenu === 'appearance'" class="appearance-content">
        <div class="content-header">
          <div class="content-title">外观设置</div>
        </div>
        
        <div class="content-body">
          <div class="form-section">
            <div class="section-title">主题设置</div>
            
            <el-form-item label="主题模式">
              <el-radio-group v-model="appearanceForm.theme">
                <el-radio label="light">浅色主题</el-radio>
                <el-radio label="dark">深色主题</el-radio>
                <el-radio label="auto">跟随系统</el-radio>
              </el-radio-group>
            </el-form-item>
            
            <el-form-item label="主题颜色">
              <el-radio-group v-model="appearanceForm.primaryColor">
                <el-radio label="#409EFF">默认蓝</el-radio>
                <el-radio label="#67C23A">成功绿</el-radio>
                <el-radio label="#E6A23C">警告橙</el-radio>
                <el-radio label="#F56C6C">危险红</el-radio>
              </el-radio-group>
            </el-form-item>
          </div>
          
          <div class="form-section">
            <div class="section-title">字体设置</div>
            
            <el-form-item label="字体大小">
              <el-slider 
                v-model="appearanceForm.fontSize" 
                :min="12" 
                :max="20" 
                :step="1"
                :marks="{12: '小', 14: '标准', 16: '中', 18: '大', 20: '特大'}"
              />
            </el-form-item>
            
            <el-form-item label="字体族">
              <el-select v-model="appearanceForm.fontFamily" placeholder="选择字体">
                <el-option label="默认字体" value="default" />
                <el-option label="微软雅黑" value="'Microsoft YaHei'" />
                <el-option label="苹方" value="'PingFang SC'" />
                <el-option label="思源黑体" value="'Source Han Sans CN'" />
              </el-select>
            </el-form-item>
          </div>
          
          <div class="form-section">
            <div class="section-title">聊天界面设置</div>
            
            <div class="switch-group">
              <div class="switch-info">
                <div class="switch-label">显示时间戳</div>
                <div class="switch-description">在消息旁边显示发送时间</div>
              </div>
              <el-switch v-model="appearanceForm.showTimestamp" />
            </div>
            
            <div class="switch-group">
              <div class="switch-info">
                <div class="switch-label">紧凑模式</div>
                <div class="switch-description">减少消息间距，显示更多内容</div>
              </div>
              <el-switch v-model="appearanceForm.compactMode" />
            </div>
          </div>
          
          <div class="form-section">
            <el-button type="primary" @click="saveAppearanceSettings">保存设置</el-button>
            <el-button @click="resetAppearanceSettings">重置</el-button>
          </div>
        </div>
      </div>
      
      <!-- 关于页面 -->
      <div v-if="activeMenu === 'about'" class="about-content">
        <div class="content-header">
          <div class="content-title">关于</div>
        </div>
        
        <div class="content-body">
          <div class="about-section">
            <div class="app-info">
              <div class="app-logo">
                <el-icon size="48"><ChatDotRound /></el-icon>
              </div>
              <div class="app-details">
                <h3>Pulse Chat</h3>
                <p>版本 1.0.0</p>
                <p>一个现代化的实时聊天应用</p>
              </div>
            </div>
            
            <div class="info-section">
              <div class="section-title">应用信息</div>
              
              <div class="info-item">
                <span class="info-label">开发团队：</span>
                <span class="info-value">Pulse Chat Team</span>
              </div>
              
              <div class="info-item">
                <span class="info-label">技术栈：</span>
                <span class="info-value">Vue 3, Element Plus, WebSocket</span>
              </div>
              
              <div class="info-item">
                <span class="info-label">开源协议：</span>
                <span class="info-value">MIT License</span>
              </div>
            </div>
            
            <div class="info-section">
              <div class="section-title">链接</div>
              
              <div class="link-item">
                <el-icon><Link /></el-icon>
                <span>官方网站</span>
              </div>
              
              <div class="link-item">
                <el-icon><Document /></el-icon>
                <span>使用文档</span>
              </div>
              
              <div class="link-item">
                <el-icon><Service /></el-icon>
                <span>技术支持</span>
              </div>
            </div>
            
            <div class="info-section">
              <div class="section-title">检查更新</div>
              
              <div class="update-info">
                <p>当前版本：1.0.0</p>
                <p>最新版本：1.0.0</p>
                <p class="update-status">您的应用已是最新版本</p>
              </div>
              
              <el-button type="primary" @click="checkUpdate">检查更新</el-button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { 
  User, 
  Setting, 
  Lock, 
  Bell, 
  Brush, 
  InfoFilled,
  Camera,
  QrCode
} from '@element-plus/icons-vue'
import { useAuthStore } from '@/stores/auth'

const userStore = useAuthStore()
const activeMenu = ref('profile')

// 菜单项配置
const menuItems = [
  { key: 'profile', label: '个人信息', icon: 'User' },
  { key: 'account', label: '账号设置', icon: 'Setting' },
  { key: 'privacy', label: '隐私设置', icon: 'Lock' },
  { key: 'notification', label: '通知设置', icon: 'Bell' },
  { key: 'appearance', label: '外观设置', icon: 'Brush' },
  { key: 'about', label: '关于', icon: 'InfoFilled' }
]

// 切换菜单
const switchMenu = (menu) => {
  activeMenu.value = menu
}

// 个人信息表单
const profileFormRef = ref()
const profileForm = reactive({
  username: '',
  nickname: '',
  email: '',
  phone: '',
  bio: ''
})

const profileRules = {
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
    { min: 3, max: 20, message: '用户名长度在 3 到 20 个字符', trigger: 'blur' }
  ],
  email: [
    { required: true, message: '请输入邮箱', trigger: 'blur' },
    { type: 'email', message: '请输入正确的邮箱格式', trigger: 'blur' }
  ],
  phone: [
    { pattern: /^1[3-9]\d{9}$/, message: '请输入正确的手机号格式', trigger: 'blur' }
  ]
}

// 保存个人信息
const saveProfile = async () => {
  if (!profileFormRef.value) return
  
  try {
    await profileFormRef.value.validate()
    // TODO: 调用API保存个人信息
    ElMessage.success('个人信息保存成功')
  } catch (error) {
    console.error('表单验证失败:', error)
  }
}

// 重置个人信息
const resetProfile = () => {
  if (profileFormRef.value) {
    profileFormRef.value.resetFields()
  }
}

// 上传头像
const uploadAvatar = () => {
  // TODO: 实现头像上传功能
  ElMessage.info('头像上传功能开发中')
}

// 账号设置表单
const accountFormRef = ref()
const accountForm = reactive({
  currentPassword: '',
  newPassword: '',
  confirmPassword: '',
  twoFactorEnabled: false
})

const accountRules = {
  currentPassword: [
    { required: true, message: '请输入当前密码', trigger: 'blur' }
  ],
  newPassword: [
    { required: true, message: '请输入新密码', trigger: 'blur' },
    { min: 6, message: '密码长度不能少于6位', trigger: 'blur' }
  ],
  confirmPassword: [
    { required: true, message: '请确认新密码', trigger: 'blur' },
    {
      validator: (rule, value, callback) => {
        if (value !== accountForm.newPassword) {
          callback(new Error('两次输入的密码不一致'))
        } else {
          callback()
        }
      },
      trigger: 'blur'
    }
  ]
}

// 修改密码
const changePassword = async () => {
  if (!accountFormRef.value) return
  
  try {
    await accountFormRef.value.validate()
    // TODO: 调用API修改密码
    ElMessage.success('密码修改成功')
    accountFormRef.value.resetFields()
  } catch (error) {
    console.error('表单验证失败:', error)
  }
}

// 隐私设置表单
const privacyForm = reactive({
  showOnlineStatus: true,
  showLastSeen: true,
  showProfile: true,
  allowStrangerMessages: false,
  sendReadReceipts: true
})

// 保存隐私设置
const savePrivacySettings = () => {
  // TODO: 调用API保存隐私设置
  ElMessage.success('隐私设置保存成功')
}

// 通知设置表单
const notificationForm = reactive({
  enableDesktop: true,
  messageNotifications: true,
  groupNotifications: true,
  friendRequestNotifications: true,
  messageSound: true,
  volume: 80
})

// 保存通知设置
const saveNotificationSettings = () => {
  // TODO: 调用API保存通知设置
  ElMessage.success('通知设置保存成功')
}

// 外观设置表单
const appearanceForm = reactive({
  theme: 'light',
  primaryColor: '#409EFF',
  fontSize: 14,
  fontFamily: 'default',
  showTimestamp: true,
  compactMode: false
})

// 保存外观设置
const saveAppearanceSettings = () => {
  // TODO: 调用API保存外观设置
  ElMessage.success('外观设置保存成功')
}

// 重置外观设置
const resetAppearanceSettings = () => {
  appearanceForm.theme = 'light'
  appearanceForm.primaryColor = '#409EFF'
  appearanceForm.fontSize = 14
  appearanceForm.fontFamily = 'default'
  appearanceForm.showTimestamp = true
  appearanceForm.compactMode = false
  ElMessage.info('外观设置已重置')
}

// 检查更新
const checkUpdate = () => {
  // TODO: 实现检查更新功能
  ElMessage.info('正在检查更新...')
}

// 初始化表单数据
onMounted(() => {
  // 从用户store加载个人信息
  if (userStore.user) {
    profileForm.username = userStore.user.username || ''
    profileForm.nickname = userStore.user.nickname || ''
    profileForm.email = userStore.user.email || ''
    profileForm.phone = userStore.user.phone || ''
    profileForm.bio = userStore.user.bio || ''
  }
})
</script>

<style lang="scss" scoped>
.settings-container {
  height: 100vh;
  display: flex;
  background: #f5f5f5;
}

// 左侧菜单
.settings-sidebar {
  width: 250px;
  background: white;
  border-right: 1px solid #e6e6e6;
  display: flex;
  flex-direction: column;
}

.sidebar-header {
  padding: 20px;
  border-bottom: 1px solid #e6e6e6;
}

.sidebar-title {
  font-size: 20px;
  font-weight: 500;
  color: #333;
}

.menu-list {
  flex: 1;
  padding: 10px 0;
}

.menu-item {
  padding: 12px 20px;
  cursor: pointer;
  transition: background 0.3s;
  display: flex;
  align-items: center;
  gap: 12px;
  color: #666;
  font-size: 14px;
  
  &:hover {
    background: #f8f9fa;
  }
  
  &.active {
    background: #e3f2fd;
    color: #409EFF;
    border-right: 3px solid #409EFF;
  }
}

.menu-icon {
  width: 20px;
  text-align: center;
}

// 右侧内容区域
.settings-content {
  flex: 1;
  background: white;
  overflow-y: auto;
}

.content-header {
  padding: 20px 30px;
  border-bottom: 1px solid #e6e6e6;
}

.content-title {
  font-size: 18px;
  font-weight: 500;
  color: #333;
}

.content-body {
  padding: 30px;
}

// 个人信息区域
.profile-section {
  margin-bottom: 40px;
}

.profile-header {
  display: flex;
  align-items: center;
  margin-bottom: 30px;
}

.profile-avatar {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  background: #409EFF;
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 32px;
  font-weight: bold;
  margin-right: 20px;
  position: relative;
  cursor: pointer;
}

.avatar-upload {
  position: absolute;
  bottom: 0;
  right: 0;
  background: #409EFF;
  color: white;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  border: 2px solid white;
  cursor: pointer;
}

.profile-info h3 {
  font-size: 20px;
  margin-bottom: 5px;
  color: #333;
}

.profile-info p {
  color: #666;
  font-size: 14px;
}

// 表单样式
.form-section {
  margin-bottom: 30px;
}

.section-title {
  font-size: 16px;
  font-weight: 500;
  margin-bottom: 20px;
  color: #333;
  padding-bottom: 10px;
  border-bottom: 1px solid #e6e6e6;
}

// 开关样式
.switch-group {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 15px 0;
  border-bottom: 1px solid #f0f0f0;
}

.switch-info {
  flex: 1;
}

.switch-label {
  font-size: 14px;
  color: #333;
  margin-bottom: 4px;
}

.switch-description {
  font-size: 12px;
  color: #999;
}

// 两步验证设置
.two-factor-setup {
  margin-top: 20px;
  padding: 20px;
  background: #f8f9fa;
  border-radius: 4px;
}

.qr-code-placeholder {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 200px;
  background: white;
  border: 2px dashed #e6e6e6;
  border-radius: 4px;
  margin-top: 20px;
  color: #999;
  
  .el-icon {
    font-size: 48px;
    margin-bottom: 10px;
  }
}

// 关于页面样式
.about-section {
  max-width: 600px;
}

.app-info {
  display: flex;
  align-items: center;
  margin-bottom: 40px;
  padding: 20px;
  background: #f8f9fa;
  border-radius: 4px;
}

.app-logo {
  margin-right: 20px;
  color: #409EFF;
}

.app-details h3 {
  font-size: 24px;
  margin-bottom: 5px;
  color: #333;
}

.app-details p {
  color: #666;
  margin-bottom: 5px;
}

.info-section {
  margin-bottom: 30px;
}

.info-item {
  display: flex;
  margin-bottom: 10px;
  
  .info-label {
    min-width: 100px;
    color: #666;
  }
  
  .info-value {
    color: #333;
  }
}

.link-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 0;
  color: #409EFF;
  cursor: pointer;
  
  &:hover {
    color: #66b1ff;
  }
}

.update-info {
  margin-bottom: 20px;
  
  p {
    margin-bottom: 5px;
    color: #666;
  }
  
  .update-status {
    color: #67C23A;
    font-weight: 500;
  }
}

// 响应式设计
@media (max-width: 768px) {
  .settings-container {
    flex-direction: column;
  }
  
  .settings-sidebar {
    width: 100%;
    height: auto;
    border-right: none;
    border-bottom: 1px solid #e6e6e6;
  }
  
  .menu-list {
    display: flex;
    overflow-x: auto;
    padding: 0 10px;
  }
  
  .menu-item {
    white-space: nowrap;
    padding: 10px 15px;
  }
  
  .content-body {
    padding: 20px;
  }
}
</style>