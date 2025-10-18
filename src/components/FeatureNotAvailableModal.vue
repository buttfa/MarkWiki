<template>
  <div class="modal-overlay" v-if="props.visible" @click="close">
    <div class="modal-container" @click.stop>
      <p class="modal-content">{{ props.message || '功能将在后续实现' }}</p>
      <div class="button-group">
        <button class="btn ok-btn" @click="close">
          OK
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// 定义props
const props = defineProps<{
  visible: boolean;
  message?: string;
}>();

// 定义emit
const emit = defineEmits<{
  (e: 'close'): void;
}>();

// 关闭弹窗
const close = () => {
  emit('close');
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
  text-align: center;
}

.modal-content {
  margin-bottom: 1.5rem;
  color: #000000;
  line-height: 1.5;
  font-size: 1rem;
}

.button-group {
  display: flex;
  justify-content: center;
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

.ok-btn {
  background-color: transparent;
  color: #9333ea;
  /* 紫色，与UI风格匹配 */
}

.ok-btn:hover {
  background-color: #f5f5f5;
}
</style>