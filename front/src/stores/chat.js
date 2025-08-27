import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { io } from 'socket.io-client'

export const useChatStore = defineStore('chat', () => {
  const socket = ref(null)
  const contacts = ref([])
  const currentContact = ref(null)
  const messages = ref([])
  const groups = ref([])
  const unreadCount = ref(0)
  const onlineUsers = ref(new Set())

  // 计算属性
  const sortedContacts = computed(() => {
    return contacts.value.sort((a, b) => {
      // 在线用户排在前面
      if (onlineUsers.value.has(a.id) && !onlineUsers.value.has(b.id)) return -1
      if (!onlineUsers.value.has(a.id) && onlineUsers.value.has(b.id)) return 1
      // 按最后消息时间排序
      return new Date(b.lastMessageTime) - new Date(a.lastMessageTime)
    })
  })

  const currentMessages = computed(() => {
    if (!currentContact.value) return []
    return messages.value.filter(msg => 
      (msg.senderId === currentContact.value.id || msg.receiverId === currentContact.value.id)
    )
  })

  // WebSocket 连接
  const connectSocket = (token) => {
    socket.value = io('ws://localhost:8080', {
      auth: {
        token
      }
    })

    socket.value.on('connect', () => {
      console.log('WebSocket connected')
    })

    socket.value.on('disconnect', () => {
      console.log('WebSocket disconnected')
    })

    socket.value.on('message', (message) => {
      addMessage(message)
    })

    socket.value.on('userOnline', (userId) => {
      onlineUsers.value.add(userId)
    })

    socket.value.on('userOffline', (userId) => {
      onlineUsers.value.delete(userId)
    })

    socket.value.on('typing', (data) => {
      // 处理打字状态
      console.log('User typing:', data)
    })
  }

  // 断开连接
  const disconnectSocket = () => {
    if (socket.value) {
      socket.value.disconnect()
      socket.value = null
    }
  }

  // 获取联系人列表
  const fetchContacts = async () => {
    try {
      const response = await fetch('/api/contacts')
      const data = await response.json()
      contacts.value = data
    } catch (error) {
      console.error('Failed to fetch contacts:', error)
    }
  }

  // 获取消息历史
  const fetchMessages = async (contactId) => {
    try {
      const response = await fetch(`/api/messages/${contactId}`)
      const data = await response.json()
      
      // 过滤并添加消息
      const contactMessages = data.filter(msg => 
        msg.senderId === contactId || msg.receiverId === contactId
      )
      
      // 合并消息并去重
      const messageIds = new Set(messages.value.map(m => m.id))
      contactMessages.forEach(msg => {
        if (!messageIds.has(msg.id)) {
          messages.value.push(msg)
        }
      })
    } catch (error) {
      console.error('Failed to fetch messages:', error)
    }
  }

  // 发送消息
  const sendMessage = (content, type = 'text') => {
    if (!currentContact.value || !socket.value) return

    const message = {
      id: Date.now().toString(),
      content,
      type,
      senderId: 'currentUser', // 应该从 auth store 获取
      receiverId: currentContact.value.id,
      timestamp: new Date().toISOString(),
      status: 'sending'
    }

    addMessage(message)
    
    socket.value.emit('message', {
      ...message,
      receiverId: currentContact.value.id
    })
  }

  // 添加消息
  const addMessage = (message) => {
    messages.value.push(message)
    
    // 更新联系人最后消息时间
    const contact = contacts.value.find(c => 
      c.id === message.senderId || c.id === message.receiverId
    )
    if (contact) {
      contact.lastMessage = message.content
      contact.lastMessageTime = message.timestamp
      
      // 如果不是当前聊天，增加未读数
      if (currentContact.value?.id !== message.senderId && message.senderId !== 'currentUser') {
        contact.unreadCount = (contact.unreadCount || 0) + 1
        unreadCount.value++
      }
    }
  }

  // 设置当前联系人
  const setCurrentContact = (contact) => {
    currentContact.value = contact
    
    // 清除未读数
    if (contact.unreadCount > 0) {
      unreadCount.value -= contact.unreadCount
      contact.unreadCount = 0
    }
    
    // 获取消息历史
    fetchMessages(contact.id)
  }

  // 搜索联系人
  const searchContacts = (query) => {
    if (!query) return sortedContacts.value
    
    return sortedContacts.value.filter(contact =>
      contact.name.toLowerCase().includes(query.toLowerCase()) ||
      contact.email?.toLowerCase().includes(query.toLowerCase())
    )
  }

  // 获取群组列表
  const fetchGroups = async () => {
    try {
      const response = await fetch('/api/groups')
      const data = await response.json()
      groups.value = data
    } catch (error) {
      console.error('Failed to fetch groups:', error)
    }
  }

  // 创建群组
  const createGroup = async (groupData) => {
    try {
      const response = await fetch('/api/groups', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(groupData)
      })
      
      if (response.ok) {
        const newGroup = await response.json()
        groups.value.push(newGroup)
        return { success: true, group: newGroup }
      }
    } catch (error) {
      console.error('Failed to create group:', error)
    }
    return { success: false }
  }

  return {
    socket,
    contacts,
    currentContact,
    messages,
    groups,
    unreadCount,
    onlineUsers,
    sortedContacts,
    currentMessages,
    connectSocket,
    disconnectSocket,
    fetchContacts,
    fetchMessages,
    sendMessage,
    addMessage,
    setCurrentContact,
    searchContacts,
    fetchGroups,
    createGroup
  }
})