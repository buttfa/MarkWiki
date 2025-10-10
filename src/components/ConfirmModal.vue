<template>
  <div class="modal-overlay" v-if="props.visible" @click="handleOverlayClick">
    <div class="modal-container" @click.stop>
      <h2 class="modal-title">{{ props.title }}</h2>
      
      <div class="modal-content">
        <p>{{ props.message }}</p>
      </div>
      
      <div class="button-group">
        <button 
          v-if="props.confirmText !== ''"
          class="btn confirm-btn"
          @click="handleConfirm"
          :disabled="props.isLoading"
        >
          <span v-if="!props.isLoading">{{ props.confirmText || '确定' }}</span>
          <span v-else class="loading-spinner">
            <span class="spinner"></span> {{ props.loadingText || '处理中...' }}
          </span>
        </button>
        <button 
          v-if="props.cancelText !== ''"
          class="btn cancel-btn"
          @click="handleCancel"
          :disabled="props.isLoading"
        >
          {{ props.cancelText || '取消' }}
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
  title: string;
  message: string;
  confirmText?: string;
  cancelText?: string;
  isLoading?: boolean;
  loadingText?: string;
}>();

// 定义emit
const emit = defineEmits<{
  (e: 'confirm'): void;
  (e: 'cancel'): void;
}>();

// 处理确认按钮点击
const handleConfirm = () => {
  emit('confirm');
};

// 处理取消按钮点击
const handleCancel = () => {
  emit('cancel');
};

// 处理遮罩层点击，点击遮罩层关闭弹窗
const handleOverlayClick = () => {
  emit('cancel');
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
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-container {
  background-color: white;
  border-radius: 8px;
  padding: 24px;
  width: 90%;
  max-width: 400px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.modal-title {
  font-size: 18px;
  font-weight: bold;
  margin-bottom: 20px;
  text-align: center;
}

/* 当只有一个按钮时，居中显示 */
.button-group:has(.confirm-btn):not(:has(.cancel-btn)) {
  align-items: center;
}

.button-group:has(.confirm-btn):not(:has(.cancel-btn)) .confirm-btn {
  width: 100px;
}

.modal-content {
  margin-bottom: 24px;
  text-align: center;
  color: #333;
  line-height: 1.5;
}

.button-group {
  display: flex;
  flex-direction: column;
  gap: 10px;
  min-height: 40px; /* 确保即使没有按钮，也有足够的空间 */
}

.btn {
  padding: 10px 16px;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.confirm-btn {
  background-color: black;
  color: white;
}

.confirm-btn:hover:not(:disabled) {
  background-color: #333;
}

.confirm-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.loading-spinner {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.spinner {
  width: 16px;
  height: 16px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-radius: 50%;
  border-top-color: white;
  animation: spin 1s ease-in-out infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.cancel-btn {
  background-color: #f5f5f5;
  color: #333;
}

.cancel-btn:hover:not(:disabled) {
  background-color: #e0e0e0;
}

.cancel-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
</style>