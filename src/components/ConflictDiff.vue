<script setup lang="ts">
import { ref, watch } from 'vue';

const props = defineProps<{
  base: string; // 基础版本内容
  local: string; // 本地修改内容
  remote: string; // 远程修改内容
}>();

const emit = defineEmits<{
  (e: 'update:resolved', value: string): void;
}>();

// 解析为行数组
const baseLines = ref(props.base.split('\n'));
const localLines = ref(props.local.split('\n'));
const remoteLines = ref(props.remote.split('\n'));
const resolvedLines = ref([...localLines.value]); // 初始使用本地版本


// 当输入变化时更新解析结果
watch(
  () => [props.base, props.local, props.remote],
  () => {
    baseLines.value = props.base.split('\n');
    localLines.value = props.local.split('\n');
    remoteLines.value = props.remote.split('\n');
    resolvedLines.value = [...localLines.value];
  },
  { deep: true }
);

// 当编辑后内容变化时通知父组件
watch(
  () => resolvedLines.value,
  () => {
    emit('update:resolved', resolvedLines.value.join('\n'));
  },
  { deep: true }
);

// 切换行的来源（本地/远程）
const useLineFrom = (index: number, source: 'local' | 'remote') => {
  resolvedLines.value[index] = source === 'local' 
    ? localLines.value[index] || '' 
    : remoteLines.value[index] || '';
};
</script>

<template>
  <div class="diff-container">
    <!-- 表头 -->
    <div class="diff-header">
      <div class="diff-column">基础版本</div>
      <div class="diff-column">本地修改 (你的更改)</div>
      <div class="diff-column">远程修改 (他人更改)</div>
      <div class="diff-column">解决结果 (可编辑)</div>
    </div>

    <!-- 内容行 -->
    <div class="diff-rows">
      <div 
        v-for="i in Math.max(baseLines.length, localLines.length, remoteLines.length)" 
        :key="i" 
        class="diff-row"
      >
        <!-- 基础版本行 -->
        <div class="diff-cell base-line">
          <span>{{ baseLines[i-1] || '' }}</span>
        </div>

        <!-- 本地版本行 -->
        <div class="diff-cell local-line" :class="{ modified: localLines[i-1] !== baseLines[i-1] }">
          <span>{{ localLines[i-1] || '' }}</span>
        </div>

        <!-- 远程版本行 -->
        <div class="diff-cell remote-line" :class="{ modified: remoteLines[i-1] !== baseLines[i-1] }">
          <span>{{ remoteLines[i-1] || '' }}</span>
        </div>

        <!-- 解决结果行 -->
        <div class="diff-cell resolved-line">
          <input
            v-model="resolvedLines[i-1]"
            :placeholder="`行 ${i}`"
          >
          <div class="line-actions">
            <button @click="useLineFrom(i-1, 'local')" v-if="localLines[i-1]">用本地</button>
            <button @click="useLineFrom(i-1, 'remote')" v-if="remoteLines[i-1]">用远程</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.diff-container {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.diff-header {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr 1.5fr;
  font-weight: bold;
  padding: 0.5rem;
  background-color: #f5f5f5;
  border-bottom: 1px solid #ddd;
}

.diff-rows {
  flex: 1;
  overflow: auto;
}

.diff-row {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr 1.5fr;
  border-bottom: 1px solid #eee;
}

.diff-cell {
  padding: 0.3rem 0.5rem;
  min-height: 1.5rem;
  white-space: pre-wrap;
  font-family: monospace;
}

.base-line {
  background-color: #f0f0f0;
  color: #666;
}

.local-line.modified {
  background-color: #e6ffed;
  border-left: 3px solid #42b983;
}

.remote-line.modified {
  background-color: #fff3e0;
  border-left: 3px solid #ff9800;
}

.resolved-line {
  background-color: #fafafa;
  position: relative;
}

.resolved-line input {
  width: 100%;
  background: transparent;
  border: 1px solid transparent;
  padding: 0.2rem;
  border-radius: 3px;
  font-family: monospace;
}

.resolved-line input:focus {
  outline: none;
  border-color: #42b983;
}

.line-actions {
  position: absolute;
  right: 0.5rem;
  top: 0.2rem;
  display: flex;
  gap: 0.3rem;
}

.line-actions button {
  font-size: 0.7rem;
  padding: 0.1rem 0.3rem;
  border: none;
  border-radius: 2px;
  cursor: pointer;
  background-color: #eee;
}
</style>