<template>
  <div class="editor-view">
    <div class="editor-header">
      <h1>{{ fileName }}</h1>
      <button @click="saveFile" class="save-btn">保存</button>
    </div>
    <mavon-editor
        v-model="content"
        :toolbars="toolbars"
        @change="handleChange"
        ref="mdEditor"
        :subfield="true"
        :default-open="'preview'"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api';

// 路由相关
const route = useRoute();
const router = useRouter();

// 编辑器内容
const content = ref('');
const isDirty = ref(false);
let debounceTimer: ReturnType<typeof setTimeout> | null = null;

// 计算属性
const wikiName = computed(() => route.params.wikiName as string);
const filePath = computed(() => route.params.filePath as string);
const fileName = computed(() => {
  // 从路径中提取文件名
  const parts = filePath.value.split('/');
  return parts[parts.length - 1] || '未命名文件';
});

// 编辑器配置
const toolbars = {
  bold: true,
  italic: true,
  header: true,
  underline: true,
  strikethrough: true,
  mark: true,
  '`': true,
  subscript: true,
  quote: true,
  ol: true,
  ul: true,
  link: true,
  imagelink: true,
  code: true,
  table: true,
  fullscreen: true,
  readmodel: true,
  htmlcode: true,
  help: true,
  undo: true,
  redo: true,
  trash: true,
  save: false, // 禁用编辑器自带的保存按钮，使用我们自己的保存按钮
  navigation: true,
  alignleft: true,
  aligncenter: true,
  alignright: true,
  subfield: true,
  preview: true
};

// 加载文件内容
const loadFile = async () => {
  try {
    // 添加参数验证
    if (!wikiName.value) {
      throw new Error('知识库名称不能为空');
    }
    
    // 标准化文件路径，特别是在Windows环境中
    let normalizedFilePath = filePath.value;
    // 处理空路径或根路径的情况
    if (!normalizedFilePath || normalizedFilePath === '/') {
      normalizedFilePath = '';
    }
    
    console.log('尝试加载文件:', { wikiName: wikiName.value, filePath: normalizedFilePath });
    
    const result = await invoke<string>('read_file', {
      wikiName: wikiName.value,  // 修改为小驼峰命名法，与Tauri期望保持一致
      filePath: normalizedFilePath   // 修改为小驼峰命名法，与Tauri期望保持一致
    });
    
    content.value = result;

    console.log('content 实际值：', content.value); // 确认此处有内容
    
    isDirty.value = false;
    console.log('文件加载成功，内容长度:', result.length);
  } catch (error) {
    console.error('加载文件失败:', error);
    // 提供更详细的错误信息
    const errorMessage = error instanceof Error ? error.message : String(error);
    alert(`加载文件失败: ${errorMessage}\n\n知识库: ${wikiName.value || '(未指定)'}\n文件路径: ${filePath.value || '(未指定)'}`);
  }
};

// 保存文件
const saveFile = async () => {
  try {
    // 添加参数验证
    if (!wikiName.value) {
      throw new Error('知识库名称不能为空');
    }
    
    // 标准化文件路径，特别是在Windows环境中
    let normalizedFilePath = filePath.value;
    // 处理空路径或根路径的情况
    if (!normalizedFilePath || normalizedFilePath === '/') {
      normalizedFilePath = '';
    }
    
    console.log('尝试保存文件:', { wikiName: wikiName.value, filePath: normalizedFilePath, content_length: content.value.length });
    
    await invoke('save_file', {
      wikiName: wikiName.value,  // 修改为小驼峰命名法，与Tauri期望保持一致
      filePath: normalizedFilePath,  // 修改为小驼峰命名法，与Tauri期望保持一致
      content: content.value
    });
    
    isDirty.value = false;
    console.log('文件保存成功');
    alert('保存成功');
  } catch (error) {
    console.error('保存文件失败:', error);
    // 提供更详细的错误信息
    const errorMessage = error instanceof Error ? error.message : String(error);
    alert(`保存文件失败: ${errorMessage}\n\n知识库: ${wikiName.value || '(未指定)'}\n文件路径: ${filePath.value || '(未指定)'}`);
  }
};

// 处理内容变化
const handleChange = () => {
  isDirty.value = true;
  
  // 防抖自动保存（可选）
  if (debounceTimer) {
    clearTimeout(debounceTimer);
  }
  debounceTimer = setTimeout(() => {
    // 可以在这里添加自动保存逻辑
  }, 2000);
};

// 监听页面关闭事件，提示保存
window.addEventListener('beforeunload', (event) => {
  if (isDirty.value) {
    event.preventDefault();
    event.returnValue = '您有未保存的内容，确定要离开吗？';
  }
});

// 监听路由参数变化，重新加载文件内容
watch([wikiName, filePath], ([newWikiName, newFilePath]) => {
  if (newWikiName && newFilePath) {
    loadFile();
  }
}, { immediate: false });

// 组件挂载时加载文件
onMounted(() => {
  if (!wikiName.value || !filePath.value) {
    alert('缺少必要参数');
    router.push('/');
    return;
  }
  loadFile();
});
</script>

<style scoped>
.editor-view {
  height: 100vh;
  display: flex;
  flex-direction: column;
  position: relative;
  z-index: 1; /* 确保编辑器视图z-index低于弹窗 */
}

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  background-color: #f5f5f5;
  border-bottom: 1px solid #e0e0e0;
}

.editor-header h1 {
  margin: 0;
  font-size: 1.5rem;
  color: #333;
}

.save-btn {
  padding: 0.5rem 1rem;
  background-color: #4CAF50;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 1rem;
}

.save-btn:hover {
  background-color: #45a049;
}

/* 确保编辑器占满剩余空间 */
:deep(.v-note-wrapper) {
  flex: 1;
  display: flex;
  flex-direction: column;
  z-index: 1; /* 确保编辑器区域z-index低于弹窗 */
}

:deep(.v-note-op-preview) {
  height: 100%;
}

:deep(.v-note-body) {
  flex: 1;
  display: flex;
}

:deep(.v-note-edit) {
  height: 100%;
}

:deep(.v-note-preview) {
  height: 100%;
  overflow-y: auto;
}
</style>