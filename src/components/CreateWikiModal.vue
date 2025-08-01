<template>
  <div class="modal-overlay" v-if="props.visible" @click="close">
    <div class="modal-container" @click.stop>
      <h2 class="modal-title">创建一个新的知识库</h2>
      
      <div class="form-group">
        <select v-model="createType" class="form-control">
          <option value="local">在本地创建</option>
          <option value="remote">从远程仓库创建</option>
        </select>
      </div>
      
      <div class="form-group">
        <input
          type="text"
          v-model="inputValue"
          :placeholder="inputPlaceholder"
          class="form-control"
        >
        <div v-if="errorMessage" class="error-message">{{ errorMessage }}</div>
      </div>
      
      <div class="button-group">
        <button class="btn create-btn" @click="handleCreate">创建</button>
        <button class="btn cancel-btn" @click="close">退出</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

// 定义props
const props = defineProps<{
  visible: boolean;
}>();

// 定义emit
const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'success'): void;
}>();

// 状态管理
const createType = ref<'local' | 'remote'>('local');
const inputValue = ref('');
const isLoading = ref(false);
const errorMessage = ref('');

// 计算属性 - 根据创建类型显示不同的占位符
const inputPlaceholder = computed(() => {
  return createType.value === 'local'
    ? '请输入知识库名'
    : '请输入远程仓库链接';
});

// 关闭弹窗
const close = () => {
  emit('close');
  inputValue.value = '';
  errorMessage.value = '';
};

// 处理创建知识库
const handleCreate = async () => {
  if (!inputValue.value.trim()) {
    errorMessage.value = createType.value === 'local'
      ? '请输入知识库名'
      : '请输入远程仓库链接';
    return;
  }

  isLoading.value = true;
  errorMessage.value = '';

  try {
    if (createType.value === 'local') {
      // 调用本地创建接口
      await invoke('create_local_wiki', { wikiName: inputValue.value.trim() });
    } else {
      // 调用远程创建接口
      await invoke('create_remote_wiki', { remoteUrl: inputValue.value.trim() });
    }

    // 创建成功，通知父组件
    emit('success');
    close();
  } catch (error) {
    errorMessage.value = error as string;
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

.form-group {
  margin-bottom: 16px;
}

.form-control {
  width: 100%;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
  box-sizing: border-box;
}

.button-group {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.btn {
  padding: 10px 16px;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.create-btn {
  background-color: black;
  color: white;
}

.create-btn:hover {
  background-color: #333;
}

.cancel-btn {
  background-color: #f5f5f5;
  color: #333;
}

.cancel-btn:hover {
  background-color: #e0e0e0;
}

.error-message {
  color: #ff4d4f;
  font-size: 12px;
  margin-top: 4px;
}
</style>