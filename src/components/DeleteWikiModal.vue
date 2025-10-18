<template>
  <div class="modal-overlay" v-if="props.visible" @click="close">
    <div class="modal-container" @click.stop>
      <h2 class="modal-title">删除知识库</h2>
      <p class="modal-content">确定要删除知识库 "{{ props.wikiName }}" 吗？此操作不可撤销。</p>
      <div class="button-group">
        <button class="btn delete-btn" @click="handleDelete">
          删除
        </button>
        <button class="btn cancel-btn" @click="close">
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
  (e: 'delete'): void;
}>();

// 关闭弹窗
const close = () => {
  emit('close');
};

// 处理删除
const handleDelete = async () => {
  emit('delete');
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
  margin-bottom: 1rem;
  color: #333;
}

.modal-content {
  margin-bottom: 1.5rem;
  color: #666;
  line-height: 1.5;
}

.button-group {
  display: flex;
  gap: 1rem;
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

.delete-btn {
  background-color: #d32f2f;
  color: #ffffff;
}

.delete-btn:hover {
  background-color: #b71c1c;
}

.cancel-btn {
  background-color: #f5f5f5;
  color: #333333;
}

.cancel-btn:hover {
  background-color: #e0e0e0;
}
</style>