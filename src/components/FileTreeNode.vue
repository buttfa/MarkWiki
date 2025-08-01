<template>
  <!-- 文件夹项组件 -->
  <div v-if="node.is_directory" class="nav-item folder-item" @click="toggleFolder(node)">
    <svg class="icon folder-icon" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
    </svg>
    <span>{{ node.name }}</span>
    <svg class="expand-icon" :class="{ 'expanded': isExpanded }" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <polyline points="6 9 12 15 18 9"></polyline>
    </svg>
  </div>
  
  <!-- 文件项组件 -->
  <div v-else class="nav-item file-item" @click="handleFileClick(node)">
    <svg class="icon file-icon" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
      <polyline points="14 2 14 8 20 8"></polyline>
      <line x1="16" x2="8" y1="13" y2="13"></line>
      <line x1="16" x2="8" y1="17" y2="17"></line>
      <polyline points="10 9 9 9 8 9"></polyline>
    </svg>
    <span>{{ node.name }}</span>
  </div>
  
  <!-- 子文件夹/文件渲染 -->
  <div v-if="node.is_directory && isExpanded" class="folder-children">
    <FileTreeNode 
      v-for="child in node.children" 
      :key="child.path" 
      :node="child"
      :selected-wiki-name="selectedWikiName"
      :folder-expanded-states="folderExpandedStates"
      @toggle-folder="$emit('toggleFolder', $event)"
      @handle-file-click="$emit('handleFileClick', $event)"
    />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

// 定义文件节点类型
interface FileNode {
  name: string;
  is_directory: boolean;
  path: string;
  children?: FileNode[];
}

const props = defineProps<{
  node: FileNode;
  selectedWikiName: string | null;
  folderExpandedStates: Record<string, boolean>;
}>();

const emit = defineEmits<{
  (e: 'toggleFolder', node: FileNode): void;
  (e: 'handleFileClick', node: FileNode): void;
}>();

const isExpanded = computed(() => props.folderExpandedStates[props.node.path]);

const toggleFolder = (node: FileNode) => {
  emit('toggleFolder', node);
};

const handleFileClick = (node: FileNode) => {
  emit('handleFileClick', node);
};
</script>

<style scoped>
.folder-item, .file-item {
  display: flex;
  align-items: center;
  padding: 10px 20px;
  color: #000000;
  cursor: pointer;
  transition: background-color 0.2s;
}

.folder-item:hover, .file-item:hover {
  background-color: #f5f5f5;
}

.icon {
  margin-right: 10px;
  width: 18px;
  height: 18px;
  color: #666;
  flex-shrink: 0;
}

.folder-icon {
  color: #666;
}

.file-icon {
  color: #666;
}

.expand-icon {
  margin-left: auto;
  transition: transform 0.2s;
}

.expand-icon.expanded {
  transform: rotate(180deg);
}

.folder-children {
  padding-left: 20px;
}
</style>