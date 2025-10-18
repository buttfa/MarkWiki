<template>
  <div class="modal-overlay" v-if="props.visible" @click="close">
    <div class="modal-container" @click.stop>
      <h2 class="modal-title">设置知识库远程仓库</h2>
      
      <div class="form-group">
        <label for="remoteUrl">远程仓库链接</label>
        <input
          type="text"
          id="remoteUrl"
          v-model="remoteUrl"
          placeholder="请输入GitHub/GitLab等仓库链接"
          class="form-control"
        >
        <div v-if="errorMessage" class="error-message">{{ errorMessage }}</div>
      </div>
      
      <div class="button-group">
        <button class="btn confirm-btn" @click="handleConfirm" :disabled="isLoading">
          <span v-if="!isLoading">确认</span>
          <span v-else class="loading-spinner">
            <span class="spinner"></span> 设置中...
          </span>
        </button>
        <button class="btn cancel-btn" @click="close" :disabled="isLoading">
          取消
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

// 定义props
const props = defineProps<{
  visible: boolean;
  wikiName: string;
}>();

// 定义emit
const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'confirm', url: string): void;
}>();

const remoteUrl = ref('');
const isLoading = ref(false);
const errorMessage = ref('');

// 关闭弹窗
const close = () => {
  emit('close');
  remoteUrl.value = '';
  errorMessage.value = '';
};

// 处理确认
const handleConfirm = async () => {
  if (!remoteUrl.value.trim()) {
    errorMessage.value = '请输入远程仓库链接';
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
    emit('confirm', remoteUrl.value.trim());
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
  gap: 1rem;
  justify-content: flex-end;
  margin-top: 1rem;
}

.btn {
  padding: 0.75rem 1.5rem;
  border-radius: 4px;
  border: none;
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.2s;
  font-weight: 500;
}

.confirm-btn {
  background-color: #000000;
  color: #ffffff;
}

.confirm-btn:hover:not(:disabled) {
  background-color: #333333;
}

.cancel-btn {
  background-color: #f5f5f5;
  color: #333333;
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
</style>