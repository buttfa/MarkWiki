<template>
  <div class="modal-overlay" v-if="props.visible" @click="close">
    <div class="modal-container" @click.stop>
      <h2 class="modal-title">设置知识库远程仓库</h2>
      
      <div v-if="isLoadingConfig" class="loading-container">
        <div class="spinner"></div>
        <span>加载配置中...</span>
      </div>
      
      <div v-else>
        <div class="form-group">
        <input
          type="text"
          id="remoteUrl"
          v-model="remoteUrl"
          placeholder="请输入GitHub/GitLab等仓库链接"
          class="form-control"
        >
      </div>
      
        <div class="form-group">
        <input
          type="text"
          id="username"
          v-model="username"
          placeholder="请输入用户名"
          class="form-control"
        >
      </div>
      
        <div class="form-group">
        <input
          type="email"
          id="email"
          v-model="email"
          placeholder="请输入邮箱"
          class="form-control"
        >
      </div>
      
        <div class="form-group">
        <input
          type="password"
          id="password"
          v-model="password"
          placeholder="请输入密码"
          class="form-control"
        >
      </div>
      
        <div v-if="errorMessage" class="error-message">{{ errorMessage }}</div>
      </div>
      
      <div class="button-group">
        <button class="btn confirm-btn" @click="handleConfirm" :disabled="isLoading || isLoadingConfig">
          <span v-if="!isLoading">确认</span>
          <span v-else class="loading-spinner">
            <span class="spinner"></span> 设置中...
          </span>
        </button>
        <button class="btn cancel-btn" @click="close" :disabled="isLoading || isLoadingConfig">
          取消
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';

// 定义props
const props = defineProps<{
  visible: boolean;
  wikiName: string;
}>();

// 定义emit
const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'confirm', url: string, username: string, email: string, password: string): void;
}>();

const remoteUrl = ref('');
const username = ref('');
const email = ref('');
const password = ref('');
const isLoading = ref(false);
const errorMessage = ref('');
const isLoadingConfig = ref(false);

// 监听弹窗可见性变化，当弹窗打开时加载配置
watch(() => props.visible, async (newVal) => {
  if (newVal && props.wikiName) {
    await loadRemoteRepoConfig();
  }
});

// 加载远程仓库配置
const loadRemoteRepoConfig = async () => {
  if (!props.wikiName) return;
  
  isLoadingConfig.value = true;
  try {
    const [remoteUrlValue, usernameValue, emailValue] = await invoke<[string | null, string, string]>('get_remote_repo_config', {
      wikiName: props.wikiName
    });
    
    remoteUrl.value = remoteUrlValue || '';
    username.value = usernameValue;
    email.value = emailValue;
    password.value = ''; // 出于安全考虑，不加载密码
  } catch (error) {
    console.error('加载远程仓库配置失败:', error);
    // 不显示错误信息，只是无法加载配置
  } finally {
    isLoadingConfig.value = false;
  }
};

// 关闭弹窗
const close = () => {
  emit('close');
  remoteUrl.value = '';
  username.value = '';
  email.value = '';
  password.value = '';
  errorMessage.value = '';
};

// 处理确认
const handleConfirm = async () => {
  if (!remoteUrl.value.trim()) {
    errorMessage.value = '请输入远程仓库链接';
    return;
  }
  
  if (!username.value.trim()) {
    errorMessage.value = '请输入用户名';
    return;
  }
  
  if (!email.value.trim()) {
    errorMessage.value = '请输入邮箱';
    return;
  }
  
  // 简单的邮箱格式验证
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  if (!emailRegex.test(email.value.trim())) {
    errorMessage.value = '请输入有效的邮箱地址';
    return;
  }
  
  // 简单的URL格式验证
  try {
    new URL(remoteUrl.value.trim());
    errorMessage.value = '';
  } catch {
    errorMessage.value = '请输入有效的URL';
    return;
  }
  
  isLoading.value = true;
  try {
    emit('confirm', remoteUrl.value.trim(), username.value.trim(), email.value.trim(), password.value.trim());
  } finally {
    isLoading.value = false;
  }
};
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-container {
  background-color: #ffffff;
  border-radius: 8px;
  padding: 2rem;
  width: 90%;
  max-width: 400px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.modal-title {
    font-size: 1.25rem;
    font-weight: 600;
    margin-bottom: 1.5rem;
    color: #333;
    text-align: center;
  }

.form-group {
  margin-bottom: 1.5rem;
}

label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: #333;
}

.form-control {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 1rem;
  transition: border-color 0.2s;
  box-sizing: border-box;
}

.form-control:focus {
  outline: none;
  border-color: #000000;
}

.error-message {
  color: #d32f2f;
  font-size: 0.875rem;
  margin-top: 0.5rem;
}

.button-group {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin-top: 1rem;
}

.btn {
  padding: 10px 16px;
  border-radius: 4px;
  border: none;
  font-size: 14px;
  cursor: pointer;
  transition: background-color 0.2s;
  font-weight: 500;
  width: 100%;
}

.confirm-btn {
  background-color: black;
  color: white;
}

.confirm-btn:hover:not(:disabled) {
  background-color: #333;
}

.cancel-btn {
  background-color: #f5f5f5;
  color: #333;
}

.cancel-btn:hover:not(:disabled) {
  background-color: #e0e0e0;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.loading-spinner {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.spinner {
  width: 16px;
  height: 16px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-radius: 50%;
  border-top-color: #ffffff;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.loading-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 2rem 0;
  color: #666;
}

.loading-container .spinner {
  width: 24px;
  height: 24px;
  border: 2px solid rgba(0, 0, 0, 0.1);
  border-radius: 50%;
  border-top-color: #000000;
  animation: spin 0.8s linear infinite;
  margin-bottom: 1rem;
}
</style>