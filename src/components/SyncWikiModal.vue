<template>
  <div class="modal-overlay" v-if="props.visible" @click="close">
    <div class="modal-container" @click.stop>
      <h2 class="modal-title">同步知识库</h2>
      
      <div class="sync-info">
        <p>是否同步知识库 "{{ props.wikiName }}"？</p>
      </div>
      
      <div class="button-group">
        <button class="btn confirm-btn" @click="confirmSync">
          确定
        </button>
        <button class="btn cancel-btn" @click="close">
          取消
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// 不需要ref导入，因为我们不再使用响应式数据

// 定义props
const props = defineProps<{
  visible: boolean;
  wikiName: string;
}>();

// 定义emit
const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'show-feature-notice'): void;
}>();

// 显示功能后续实现提示
const confirmSync = () => {
  // 触发事件让父组件显示统一的功能后续实现提示
  emit('show-feature-notice');
};

// 关闭弹窗
const close = () => {
  emit('close');
};
</script>

<style scoped>
.sync-info p {
  text-align: center;
  margin: 1rem 0;
}

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
    font-size: 18px;
    font-weight: 600;
    margin-bottom: 20px;
    color: #333;
    text-align: center;
  }

.sync-info {
  margin-bottom: 1.5rem;
}

.sync-info p {
  color: #666;
  line-height: 1.5;
  margin-bottom: 1rem;
}

.sync-progress {
  margin-bottom: 1rem;
}

.progress-bar {
  width: 100%;
  height: 8px;
  background-color: #f0f0f0;
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 0.5rem;
}

.progress-fill {
  height: 100%;
  background-color: #000000;
  transition: width 0.3s ease;
}

.progress-text {
  font-size: 0.875rem;
  color: #666;
  text-align: right;
}

.sync-message {
  font-size: 0.875rem;
  color: #666;
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
}

.confirm-btn {
  background-color: black;
  color: white;
}

.confirm-btn:hover {
  background-color: #333;
}

.cancel-btn {
  background-color: #f5f5f5;
  color: #333;
}

.cancel-btn:hover {
  background-color: #e0e0e0;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
</style>