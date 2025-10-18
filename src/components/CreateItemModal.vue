<template>
  <div class="modal-overlay" v-if="props.visible" @click="close">
    <div class="modal-container" @click.stop>
      <h2 class="modal-title">新增文件/文件夹</h2>
      
      <div class="form-group">
        <select v-model="itemType" class="form-control">
          <option value="file">文件</option>
          <option value="folder">文件夹</option>
        </select>
      </div>
      
      <div class="form-group">
        <label for="itemName">名称</label>
        <input
          type="text"
          id="itemName"
          v-model="itemName"
          :placeholder="itemType === 'file' ? '请输入文件名（带.md后缀）' : '请输入文件夹名'"
          class="form-control"
        >
        <div v-if="errorMessage" class="error-message">{{ errorMessage }}</div>
      </div>
      
      <div class="button-group">
        <button class="btn create-btn" @click="handleCreate" :disabled="isLoading">
          <span v-if="!isLoading">创建</span>
          <span v-else class="loading-spinner">
            <span class="spinner"></span> 创建中...
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
  parentPath?: string;
}>();

// 定义emit
const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'create', type: 'file' | 'folder', name: string, parentPath?: string): void;
}>();

const itemType = ref<'file' | 'folder'>('file');
const itemName = ref('');
const isLoading = ref(false);
const errorMessage = ref('');

// 关闭弹窗
const close = () => {
  emit('close');
  itemType.value = 'file';
  itemName.value = '';
  errorMessage.value = '';
};

// 处理创建
const handleCreate = async () => {
  if (!itemName.value.trim()) {
    errorMessage.value = '请输入名称';
    return;
  }
  
  // 文件名必须带.md后缀
  if (itemType.value === 'file' && !itemName.value.trim().endsWith('.md')) {
    errorMessage.value = '文件名必须以.md结尾';
    return;
  }
  
  // 检查名称是否包含非法字符（简化版）
  const invalidChars = /[<>"|?*\/:]/;
  if (invalidChars.test(itemName.value.trim())) {
    errorMessage.value = '名称包含非法字符';
    return;
  }
  
  isLoading.value = true;
  try {
    emit('create', itemType.value, itemName.value.trim(), props.parentPath);
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

.create-btn {
  background-color: #000000;
  color: #ffffff;
}

.create-btn:hover:not(:disabled) {
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