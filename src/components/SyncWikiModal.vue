<template>
  <div class="modal-overlay" v-if="props.visible" @click="close">
    <div class="modal-container" @click.stop>
      <h2 class="modal-title">同步知识库</h2>
      
      <div class="sync-info">
        <p>正在同步知识库 "{{ props.wikiName }}"...</p>
        <div class="sync-progress">
          <div class="progress-bar">
            <div class="progress-fill" :style="{ width: progress + '%' }"></div>
          </div>
          <span class="progress-text">{{ progress }}%</span>
        </div>
        <p v-if="syncMessage" class="sync-message">{{ syncMessage }}</p>
      </div>
      
      <div class="button-group">
        <button class="btn cancel-btn" @click="close" :disabled="isSyncing">
          {{ isSyncing ? '同步中...' : '关闭' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';

// 定义props
const props = defineProps<{
  visible: boolean;
  wikiName: string;
}>();

// 定义emit
const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'sync-start'): void;
}>();

const isSyncing = ref(false);
const progress = ref(0);
const syncMessage = ref('');
let progressInterval: number | null = null;

// 模拟进度更新
const updateProgress = () => {
  if (progress.value < 100) {
    // 随机增加进度，模拟真实同步过程
    const increment = Math.random() * 10 + 5;
    progress.value = Math.min(progress.value + increment, 100);
    
    // 更新同步消息
    if (progress.value < 30) {
      syncMessage.value = '正在连接远程仓库...';
    } else if (progress.value < 60) {
      syncMessage.value = '正在获取最新内容...';
    } else if (progress.value < 90) {
      syncMessage.value = '正在合并更改...';
    } else {
      syncMessage.value = '同步完成';
    }
  } else {
    // 同步完成
    isSyncing.value = false;
    if (progressInterval) {
      clearInterval(progressInterval);
      progressInterval = null;
    }
    // 延迟关闭，让用户看到完成状态
    setTimeout(() => {
      close();
    }, 1000);
  }
};

// 关闭弹窗
const close = () => {
  if (!isSyncing.value) {
    emit('close');
    resetState();
  }
};

// 重置状态
const resetState = () => {
  isSyncing.value = false;
  progress.value = 0;
  syncMessage.value = '';
  if (progressInterval) {
    clearInterval(progressInterval);
    progressInterval = null;
  }
};

// 开始同步
const startSync = () => {
  isSyncing.value = true;
  progress.value = 0;
  syncMessage.value = '';
  emit('sync-start');
  
  // 开始模拟进度更新
  progressInterval = window.setInterval(updateProgress, 500);
};

// 监听visible变化，自动开始同步
onMounted(() => {
  if (props.visible) {
    startSync();
  }
});

// 清理定时器
onUnmounted(() => {
  if (progressInterval) {
    clearInterval(progressInterval);
  }
});
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
  justify-content: flex-end;
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
</style>