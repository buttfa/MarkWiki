<script setup lang="ts">
const props = defineProps<{
  wikiName: string;
}>();

</script>

<template>
  <div class="workspace-container">
    <div v-if="!props.wikiName" class="no-wiki-selected">
      <svg class="alert-icon" xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="12" cy="12" r="10"></circle>
        <line x1="12" x2="12" y1="8" y2="12"></line>
        <line x1="12" x2="12.01" y1="16" y2="16"></line>
      </svg>
      <p>请先选择知识库</p>
    </div>

    <div v-else class="wiki-content">
      <div class="wiki-selection-hint">
        <svg class="info-icon" xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="10"></circle>
          <line x1="12" x2="12" y1="16" y2="12"></line>
          <line x1="12" x2="12.01" y1="8" y2="8"></line>
        </svg>
        <p>已选中知识库{{ props.wikiName }}，现可在左侧工作区选择文件继续编辑/预览</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.workspace-container {
  flex: 1;
  padding: 2rem;
  height: 100vh;
  overflow-y: auto;
  background-color: #ffffff;
  color: #000000;
}

.wiki-content {
  display: flex;
  height: 70%;
}

.no-wiki-selected {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #333;
}

.alert-icon {
  margin-bottom: 1rem;
  color: #666;
}

.wiki-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid #eee;
}

.wiki-actions {
  display: flex;
  gap: 1rem;
}

.action-btn {
  display: flex;
  align-items: center;
  background-color: #f9f9f9;
  border: 1px solid #eee;
  padding: 0.5rem 1rem;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
  color: #000000;
}

.action-btn:hover {
  background-color: #f0f0f0;
  border-color: #ddd;
}

.icon {
  margin-right: 0.5rem;
}

.wiki-files {
  margin-top: 2rem;
}

.file-list {
  list-style: none;
  padding: 0;
}

.file-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  border-bottom: 1px solid #eee;
}

.file-name {
  font-size: 1rem;
}

.file-actions {
  display: flex;
  gap: 0.5rem;
}

.file-action-btn {
  padding: 0.3rem 0.8rem;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.85rem;
  transition: all 0.2s;
}

.file-action-btn:first-child {
  background-color: #f0f0f0;
  color: #000000;
  border: 1px solid #ddd;
}

.file-action-btn:last-child {
  background-color: #f0f0f0;
  color: #000000;
  border: 1px solid #ddd;
}

.no-files {
  padding: 2rem;
  text-align: center;
  color: #333;
  background-color: #ffffff;
}

.wiki-selection-hint {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 2rem;
  margin: 0 auto;
  max-width: 500px;
  height: 100%;
  border: 1px dashed #eee;
  border-radius: 8px;
  text-align: center;
}

.info-icon {
  margin-bottom: 1rem;
  color: #666;
}

/* 移除深色模式支持 */
  .no-wiki-selected {
    color: #bdc3c7;
  }
  .wiki-header {
    border-bottom-color: #34495e;
  }
  .action-btn {
    background-color: #34495e;
    border-color: #4a6581;
    color: #ecf0f1;
  }
  .action-btn:hover {
    background-color: #4a6581;
  }
  .file-item {
    border-bottom-color: #34495e;
  }
  .no-files {
    background-color: #34495e;
    color: #bdc3c7;
  }
</style>