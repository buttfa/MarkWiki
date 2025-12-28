<script setup lang="ts">
import { ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import ConflictDiff from '@/components/ConflictDiff.vue';
import { writeTextFile } from '@tauri-apps/plugin-fs';
import { message } from '../utils/notification'; 

// 接收路由参数（冲突文件路径、冲突内容）
const route = useRoute();
const router = useRouter();
const conflictFile = ref<{
  path: string;
  baseContent: string; // 基础版本
  localContent: string; // 本地修改
  remoteContent: string; // 远程修改
}>(route.params.conflictFile as any);

// 最终解决后的内容
const resolvedContent = ref(conflictFile.value.localContent);

// 切换采用的版本
const chooseVersion = (version: 'base' | 'local' | 'remote') => {
  switch (version) {
    case 'base':
      resolvedContent.value = conflictFile.value.baseContent;
      break;
    case 'local':
      resolvedContent.value = conflictFile.value.localContent;
      break;
    case 'remote':
      resolvedContent.value = conflictFile.value.remoteContent;
      break;
  }
};

// 保存解决后的文件
const saveResolvedFile = async () => {
  try {
    await writeTextFile(conflictFile.value.path, resolvedContent.value);
    message.success('冲突已解决并保存');
    router.back(); // 返回上一页
  } catch (err) {
    message.error('保存失败：' + (err as Error).message);
  }
};
</script>

<template>
  <div class="conflict-resolver-container">
    <div class="header">
      <h1>文件冲突解析: {{ conflictFile.path.split('/').pop() }}</h1>
      <div class="actions">
        <button @click="chooseVersion('local')">采用本地版本</button>
        <button @click="chooseVersion('remote')">采用远程版本</button>
        <button @click="saveResolvedFile" class="save-btn">保存解决结果</button>
      </div>
    </div>

    <div class="diff-content">
      <ConflictDiff
        :base="conflictFile.baseContent"
        :local="conflictFile.localContent"
        :remote="conflictFile.remoteContent"
        v-model:resolved="resolvedContent"
      />
    </div>
  </div>
</template>

<style scoped>
.conflict-resolver-container {
  padding: 1rem 2rem;
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid #eee;
}

.actions button {
  margin-left: 0.5rem;
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.actions button.save-btn {
  background-color: #42b983;
  color: white;
}

.diff-content {
  flex: 1;
  overflow: auto;
  border: 1px solid #eee;
  border-radius: 4px;
}
</style>