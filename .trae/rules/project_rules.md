# 项目规则与规范

## 一、项目概述 (Project Overview)
- **项目名称**: Pulse Chat
- **核心功能**: 一个基于Web的实时聊天应用，支持单聊、群组聊天、联系人管理等功能
- **当前阶段**: 从0到1的原型设计与功能实现

## 二、技术栈 (Tech Stack)
- **前端框架**: Vue 3 (Composition API)
- **UI组件库**: Element Plus (基于Vue 3的版本)
- **编程语言**: JavaScript
- **状态管理**: Pinia
- **路由管理**: Vue Router 4
- **HTTP客户端**: Axios
- **实时通信**: WebSocket (原生API或Socket.IO客户端)
- **构建工具**: Vite
- **包管理**: npm
- **样式预处理**: SCSS
- **图标库**: Element Plus图标 或 Remix Icon
- **工具库**: dayjs (日期处理), lodash (实用函数)

## 三、项目结构 (Project Structure)
front/
│
├── public/ # 静态资源
│ └── index.html
│
├── src/
│ ├── assets/ # 静态资源（图片、字体等）
│ ├── components/ # 可复用组件
│ │ ├── common/ # 全局通用组件
│ │ ├── layout/ # 布局组件
│ │ ├── chat/ # 聊天相关组件
│ │ ├── contacts/ # 联系人相关组件
│ │ └── search/ # 搜索相关组件
│ ├── composables/ # 组合式函数
│ ├── layouts/ # 页面布局
│ │ ├── DefaultLayout.vue
│ │ └── AuthLayout.vue
│ ├── router/ # 路由配置
│ ├── stores/ # Pinia状态管理
│ │ ├── auth.js # 认证状态
│ │ ├── chat.js # 聊天状态
│ │ ├── contacts.js # 联系人状态
│ │ ├── theme.js # 主题状态
│ │ └── index.js # 聚合导出
│ ├── styles/ # 全局样式
│ │ ├── _variables.scss # 样式变量
│ │ ├── _mixins.scss # 混合宏
│ │ ├── _theme.scss # 主题样式
│ │ └── main.scss # 主样式文件
│ ├── utils/ # 工具函数
│ ├── views/ # 页面组件
│ │ ├── Login.vue # 登录页
│ │ ├── Chat.vue # 聊天主页面
│ │ └── Settings.vue # 设置页（含退出）
│ ├── App.vue
│ └── main.js
│
├── package.json
├── vite.config.js
└── index.html

## 四、编码规范 (Coding Standards)
- **代码风格**: 遵循Vue官方风格指南，使用ESLint + Prettier保证代码一致性
- **组件命名**: 多词命名（避免与HTML元素冲突），PascalCase命名组件文件
- **API调用**: 使用统一的API服务层封装所有HTTP请求
- **响应式数据**: 优先使用Composition API和ref/reactive管理状态
- **Props定义**: 使用TypeScript风格的Props定义（即使使用JS，也要详细注释）
- **事件处理**: 自定义事件使用kebab-case命名
- **样式规范**: 使用SCSS，遵循BEM命名规范（可选），组件样式使用scoped

## 五、功能规范 (Feature Standards)
### 认证流程
- **登录页面**: 用户名/密码登录，支持记住登录状态
- **权限控制**: 路由守卫保护需要认证的页面
- **Token管理**: 自动在请求头中添加认证token

### 聊天功能
- **消息类型**: 支持文本、表情、图片（后续可扩展）
- **会话管理**: 保存最近会话列表，支持会话切换
- **消息状态**: 显示发送中、已发送、已读状态
- **实时更新**: 使用WebSocket接收实时消息
- **消息持久化**: 本地存储最近消息记录

### 联系人管理
- **搜索功能**: 支持按用户名/ID搜索用户
- **添加联系人**: 发送添加请求，等待对方确认
- **删除联系人**: 移除联系人并更新相关聊天会话
- **分组管理**: 支持对联系人进行分组（好友、同事、家人等）

### 主题系统
- **主题切换**: 支持亮色/暗色主题切换
- **持久化存储**: 用户主题偏好保存在localStorage中
- **全局响应**: 所有组件响应主题变化

## 六、组件设计规范 (Component Design Standards)
- **单一职责**: 每个组件只关注一个特定功能
- **Props验证**: 详细定义Props类型和默认值
- **事件通信**: 子组件通过事件与父组件通信
- **插槽使用**: 合理使用插槽提高组件灵活性
- **文档注释**: 每个组件有清晰的注释说明用途和用法

## 七、状态管理规范 (State Management Standards)
- **Pinia存储**: 按照功能模块划分store
- **数据流**: 组件 -> Actions -> State -> 组件更新
- **持久化**: 重要状态（如用户信息、主题）需要持久化
- **模块化**: 每个store专注于单一功能领域

## 八、API交互规范 (API Interaction Standards)
- **服务封装**: 所有API请求在专门的服务模块中处理
- **错误处理**: 统一处理HTTP错误，提供用户友好提示
- **加载状态**: 异步操作时显示加载状态
- **请求拦截**: 使用axios拦截器添加认证token

## 九、主题系统规范 (Theme System Standards)
- **CSS变量**: 使用CSS变量定义主题颜色
- **类名切换**: 通过切换html上的类名实现主题变化
- **组件适配**: Element Plus组件使用CSS变量适配自定义主题
- **扩展性**: 预留多主题扩展能力

## 十、响应式设计规范 (Responsive Design Standards)
- **移动优先**: 设计时优先考虑移动端体验
- **断点设置**: 使用标准断点（sm: 640px, md: 768px, lg: 1024px, xl: 1280px）
- **弹性布局**: 使用Flex/Grid实现响应式布局
- **元素隐藏**: 在小屏幕上合理隐藏次要元素

## 十一、性能优化规范 (Performance Standards)
- **组件懒加载**: 路由级别组件使用懒加载
- **图片优化**: 使用适当格式和压缩
- **代码分割**: 合理分割代码包，减少初始加载时间
- **虚拟列表**: 长列表使用虚拟滚动

## 十二、特定规则与禁忌 (Specific Rules & Taboos)
- **严禁**: 在组件中直接写死API地址
- **避免**: 在组件中直接操作DOM，优先使用Vue的数据驱动方式
- **注意**: 所有用户输入都需要进行验证和转义，防止XSS攻击
- **要求**: 所有交互操作都需要提供反馈（加载中、成功、错误状态）
- **优先**: 使用Composition API而不是Options API

## 十三、浏览器兼容性 (Browser Compatibility)
- **主要支持**: Chrome >= 88, Firefox >= 85, Safari >= 14, Edge >= 88
- **降级方案**: 对不支持某些特性的浏览器提供基本功能