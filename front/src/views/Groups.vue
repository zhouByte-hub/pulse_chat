<template>
  <div class="groups-container">
    <!-- 群组管理头部 -->
    <div class="groups-header">
      <h1 class="header-title">群组管理</h1>
      <div class="header-actions">
        <el-button class="action-btn secondary" @click="refreshGroups">
          <i class="fas fa-sync"></i>
          刷新
        </el-button>
        <el-button type="primary" class="action-btn" @click="createGroup">
          <i class="fas fa-plus"></i>
          创建群组
        </el-button>
      </div>
    </div>
    
    <!-- 群组内容 -->
    <div class="groups-content">
      <!-- 主群组列表 -->
      <div class="groups-main">
        <div class="groups-tabs">
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
          <!-- 我的群组 -->
          <div v-if="activeTab === 'my'" class="tab-pane active">
            <div class="search-bar">
              <el-input 
                v-model="searchQuery" 
                placeholder="搜索群组..."
                prefix-icon="Search"
                clearable
              />
            </div>
            
            <div class="groups-grid">
              <div 
                v-for="group in filteredGroups" 
                :key="group.id"
                class="group-card"
                @click="enterGroup(group)"
              >
                <div class="group-header">
                  <el-avatar :size="50" :src="group.avatar">
                    {{ group.name.charAt(0) }}
                  </el-avatar>
                  <div class="group-info">
                    <div class="group-name">{{ group.name }}</div>
                    <div class="group-members">{{ group.memberCount }} 成员</div>
                  </div>
                </div>
                <div class="group-description">{{ group.description }}</div>
                <div class="group-meta">
                  <div class="group-role">{{ group.role }}</div>
                  <div class="group-time">{{ group.lastActive }}</div>
                </div>
              </div>
            </div>
          </div>
          
          <!-- 发现群组 -->
          <div v-if="activeTab === 'discover'" class="tab-pane active">
            <div class="search-bar">
              <el-input 
                v-model="discoverSearchQuery" 
                placeholder="搜索群组..."
                prefix-icon="Search"
                clearable
              />
            </div>
            
            <div class="groups-grid">
              <div 
                v-for="group in filteredDiscoverGroups" 
                :key="group.id"
                class="group-card"
              >
                <div class="group-header">
                  <el-avatar :size="50" :src="group.avatar">
                    {{ group.name.charAt(0) }}
                  </el-avatar>
                  <div class="group-info">
                    <div class="group-name">{{ group.name }}</div>
                    <div class="group-members">{{ group.memberCount }} 成员</div>
                  </div>
                </div>
                <div class="group-description">{{ group.description }}</div>
                <div class="group-actions">
                  <el-button 
                    type="primary" 
                    size="small"
                    @click="joinGroup(group)"
                  >
                    加入群组
                  </el-button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 侧边栏 -->
      <div class="groups-sidebar">
        <div class="sidebar-section">
          <h3 class="sidebar-title">群组分类</h3>
          <div class="filter-list">
            <div 
              v-for="category in categories" 
              :key="category.key"
              :class="['filter-item', { active: selectedCategory === category.key }]"
              @click="selectedCategory = category.key"
            >
              <div class="filter-info">
                <i :class="category.icon"></i>
                <span class="filter-text">{{ category.name }}</span>
              </div>
              <div class="filter-count">{{ category.count }}</div>
            </div>
          </div>
        </div>
        
        <div class="sidebar-section">
          <h3 class="sidebar-title">群组统计</h3>
          <div class="stats-grid">
            <div class="stat-item">
              <div class="stat-number">{{ groupStats.myGroups }}</div>
              <div class="stat-label">我的群组</div>
            </div>
            <div class="stat-item">
              <div class="stat-number">{{ groupStats.totalMembers }}</div>
              <div class="stat-label">总成员数</div>
            </div>
            <div class="stat-item">
              <div class="stat-number">{{ groupStats.createdGroups }}</div>
              <div class="stat-label">创建的群组</div>
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
import { ElMessage } from 'element-plus'

const router = useRouter()

// 响应式数据
const activeTab = ref('my')
const searchQuery = ref('')
const discoverSearchQuery = ref('')
const selectedCategory = ref('all')

// 标签页配置
const tabs = [
  { key: 'my', name: '我的群组' },
  { key: 'discover', name: '发现群组' }
]

// 群组分类
const categories = [
  { key: 'all', name: '全部', icon: 'fas fa-globe', count: 25 },
  { key: 'work', name: '工作', icon: 'fas fa-briefcase', count: 8 },
  { key: 'study', name: '学习', icon: 'fas fa-graduation-cap', count: 6 },
  { key: 'life', name: '生活', icon: 'fas fa-home', count: 7 },
  { key: 'game', name: '游戏', icon: 'fas fa-gamepad', count: 4 }
]

// 群组统计数据
const groupStats = reactive({
  myGroups: 12,
  totalMembers: 156,
  createdGroups: 3
})

// 模拟群组数据
const myGroups = reactive([
  {
    id: 1,
    name: '前端开发交流群',
    description: '分享前端技术，交流开发经验',
    avatar: '',
    memberCount: 45,
    role: '管理员',
    lastActive: '2小时前',
    category: 'work'
  },
  {
    id: 2,
    name: 'Vue学习小组',
    description: 'Vue.js学习与讨论',
    avatar: '',
    memberCount: 28,
    role: '成员',
    lastActive: '1天前',
    category: 'study'
  },
  {
    id: 3,
    name: '生活分享',
    description: '分享生活中的美好瞬间',
    avatar: '',
    memberCount: 67,
    role: '成员',
    lastActive: '刚刚',
    category: 'life'
  }
])

const discoverGroups = reactive([
  {
    id: 101,
    name: 'React开发者社区',
    description: 'React技术交流与分享',
    avatar: '',
    memberCount: 234,
    category: 'work'
  },
  {
    id: 102,
    name: '算法学习群',
    description: '一起学习算法，提升编程能力',
    avatar: '',
    memberCount: 89,
    category: 'study'
  },
  {
    id: 103,
    name: '游戏爱好者',
    description: '游戏讨论与组队',
    avatar: '',
    memberCount: 156,
    category: 'game'
  }
])

// 计算属性
const filteredGroups = computed(() => {
  let filtered = myGroups
  
  if (selectedCategory.value !== 'all') {
    filtered = filtered.filter(group => group.category === selectedCategory.value)
  }
  
  if (searchQuery.value) {
    filtered = filtered.filter(group => 
      group.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      group.description.toLowerCase().includes(searchQuery.value.toLowerCase())
    )
  }
  
  return filtered
})