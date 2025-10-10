<template>
  <aside class="sidebar" :class="{ 'collapsed': isSidebarCollapsed }">
  <CreateWikiModal
    :visible="isCreateWikiModalVisible"
    @close="closeCreateWikiModal"
    @success="onCreateWikiSuccess"
  />
  <div class="logo-container">
    <svg 
      class="icon sidebar-toggle" 
      @click="toggleSidebarCollapse"
      xmlns="http://www.w3.org/2000/svg" 
      width="24" 
      height="24" 
      viewBox="0 0 24 24" 
      fill="none" 
      stroke="currentColor" 
      stroke-width="2" 
      stroke-linecap="round" 
      stroke-linejoin="round"
    >
      <line x1="3" y1="6" x2="21" y2="6"></line>
      <line x1="3" y1="12" x2="21" y2="12"></line>
      <line x1="3" y1="18" x2="21" y2="18"></line>
    </svg>
    <h1 class="logo">MarkWiki</h1>
  </div>
    
    <nav class="nav-menu">
      <div class="nav-item" @click="navigateTo('/')">
        <svg class="icon" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>
          <polyline points="9 22 9 12 15 12 15 22"></polyline>
        </svg>
        <span>主页</span>
      </div>

      <div class="nav-group">
        <div class="group-title">
          <svg class="icon menu-toggle" :class="{ 'collapsed': isWikiCollapsed }" @click="toggleWikiCollapse" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="3" y1="6" x2="21" y2="6"></line>
            <line x1="3" y1="12" x2="21" y2="12"></line>
            <line x1="3" y1="18" x2="21" y2="18"></line>
          </svg>
          <span>知识库</span>
          <button class="refresh-btn" @click="refreshWikis">
            <svg class="icon" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="23 4 23 10 17 10"></polyline>
              <polyline points="1 20 1 14 7 14"></polyline>
              <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"></path>
            </svg>
          </button>
        </div>
        <Transition name="fade">
          <div key="wiki-content" class="wiki-list-container" :class="{ 'collapsed': isWikiCollapsed }">
            <div v-if="isLoading" class="loading-item">
              <div class="spinner"></div>
              <span>加载中...</span>
            </div>
            <div v-else-if="error" class="error-item">
              <svg class="icon" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="10"></circle>
                <line x1="12" x2="12" y1="8" y2="12"></line>
                <line x1="12" x2="12.01" y1="16" y2="16"></line>
              </svg>
              <span>{{ error }}</span>
            </div>
            <div v-else-if="wikis.length === 0" class="empty-item">
              <svg class="icon" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="10"></circle>
                <line x1="12" x2="12" y1="8" y2="12"></line>
                <line x1="12" x2="12.01" y1="16" y2="16"></line>
              </svg>
              <span>暂无知识库，请创建</span>
            </div>
            <div v-for="wiki in wikis" :key="wiki.name" class="wiki-item">
              <div class="nav-item" :class="{ 'active': selectedWikiName === wiki.name }" @click="selectWiki(wiki.name)">
                <svg class="icon" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"></path>
                  <path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"></path>
                </svg>
                <span>{{ wiki.name }}</span>
              </div>
              <div v-if="expandedWikiName === wiki.name" class="wiki-subitems">
                <div v-if="wiki.has_remote_repo" class="nav-item" @click="syncWiki(wiki.name)">
                  <svg class="icon" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="23 4 23 10 17 10"></polyline>
                    <polyline points="1 20 1 14 7 14"></polyline>
                    <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"></path>
                  </svg>
                  <span>同步</span>
                </div>
                <div class="nav-item" @click="setupRemoteRepo(wiki.name)">
                  <svg class="icon" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M17.5 19H9a7 7 0 1 1 6.71-9h1.79a4.5 4.5 0 1 1 0 9Z"></path>
                    <line x1="9" x2="15" y1="9" y2="15"></line>
                    <line x1="15" x2="9" y1="9" y2="15"></line>
                  </svg>
                  <span>设置远程仓库</span>
                </div>
                <div class="nav-item" @click="deleteWiki(wiki.name)">
                  <svg class="icon" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M3 6h18"></path>
                    <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"></path>
                    <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"></path>
                    <line x1="10" x2="10" y1="11" y2="17"></line>
                    <line x1="14" x2="14" y1="11" y2="17"></line>
                  </svg>
                  <span>删除</span>
                </div>
              </div>
            </div>
            <div class="nav-item create-wiki" @click="createWiki">
              <svg class="icon" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="10"></circle>
                <path d="M8 12h8"></path>
                <path d="M12 8v8"></path>
              </svg>
              <span>创建知识库</span>
            </div>
          </div>
        </Transition>
      </div>

      <div class="nav-group">
        <div class="group-title">
            <svg class="icon menu-toggle" :class="{ 'collapsed': isWorkspaceCollapsed }" @click="toggleWorkspaceCollapse" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="3" y1="6" x2="21" y2="6"></line>
              <line x1="3" y1="12" x2="21" y2="12"></line>
              <line x1="3" y1="18" x2="21" y2="18"></line>
            </svg>
            <span>工作区</span>
            <button v-if="selectedWikiName" class="refresh-btn" @click="refreshWorkspace">
              <svg class="icon" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="23 4 23 10 17 10"></polyline>
                <polyline points="1 20 1 14 7 14"></polyline>
                <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"></path>
              </svg>
            </button>
        </div>
        <Transition name="fade">
          <div key="workspace-content" class="wiki-list-container" :class="{ 'collapsed': isWorkspaceCollapsed }">
            <div v-if="!selectedWikiName" class="nav-item placeholder-item">
              <svg class="icon" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="10"></circle>
                <line x1="12" x2="12" y1="8" y2="12"></line>
                <line x1="12" x2="12.01" y1="16" y2="16"></line>
              </svg>
              <span>请先选择知识库</span>
            </div>
            <div v-else-if="isWorkspaceLoading" class="loading-item">
              <div class="spinner"></div>
              <span>加载中...</span>
            </div>
            <div v-else-if="workspaceError" class="error-item">
              <svg class="icon" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="10"></circle>
                <line x1="12" x2="12" y1="8" y2="12"></line>
                <line x1="12" x2="12.01" y1="16" y2="16"></line>
              </svg>
              <span>{{ workspaceError }}</span>
            </div>
            <div v-if="selectedWikiName">
              <FileTreeNode
                v-for="item in workspaceItems"
                :key="item.path"
                :node="item"
                :selected-wiki-name="selectedWikiName"
                :folder-expanded-states="folderExpandedStates"
                @toggle-folder="toggleFolder"
                @handle-file-click="handleFileClick"
              />
              <div class="nav-item create-item" @click="createItem">
                <svg class="icon" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <circle cx="12" cy="12" r="10"></circle>
                  <path d="M8 12h8"></path>
                  <path d="M12 8v8"></path>
                </svg>
                <span>新增文件/文件夹</span>
              </div>
            </div>
          </div>
        </Transition>
      </div>
    </nav>
  </aside>
</template>

<script setup lang="ts">
import { ref, onMounted, Ref } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from "@tauri-apps/api/core";
import FileTreeNode from './FileTreeNode.vue';
import CreateWikiModal from './CreateWikiModal.vue';

// 侧边栏折叠状态 - 在移动设备上默认收起
const isSidebarCollapsed = ref(window.innerWidth <= 768);

// 监听窗口大小变化，动态调整侧边栏状态
window.addEventListener('resize', () => {
  isSidebarCollapsed.value = window.innerWidth <= 768;
});

// 定义知识库类型
interface Wiki {
  name: string;
  has_remote_repo: boolean;
  path: string;
}

// 创建知识库弹窗相关状态和方法
const isCreateWikiModalVisible = ref(false);

// 定义文件节点类型
interface FileNode {
  name: string;
  is_directory: boolean;
  path: string;
  children?: FileNode[];
}

// 知识库数据
const wikis = ref<Wiki[]>([]);
const isLoading = ref(true);
const error = ref<string | null>(null);

// 工作区文件结构数据
const workspaceItems: Ref<FileNode[]> = ref([]);
const isWorkspaceLoading = ref(false);
const workspaceError = ref<string | null>(null);

// 刷新知识库列表
const refreshWikis = async () => {
  try {
    // 先设置为加载中状态，触发过渡效果
    isLoading.value = true;
    error.value = null;
    // 短暂延迟，确保过渡效果可见
    await new Promise(resolve => setTimeout(resolve, 100));
    const result = await invoke<Wiki[]>('get_wiki_list');
    wikis.value = result;
  } catch (err) {
    error.value = err instanceof Error ? err.message : '获取知识库列表失败';
    console.error('Failed to refresh wiki list:', err);
  } finally {
    // 延迟隐藏加载状态，确保过渡效果可见
    setTimeout(() => {
      isLoading.value = false;
    }, 100);
  }
};

// 刷新工作区文件结构
  const refreshWorkspace = async () => {
    if (!selectedWikiName.value) return;

    try {
      // 显示加载状态
      isWorkspaceLoading.value = true;
      workspaceError.value = null;
      
      // 添加延迟以确保过渡动画效果
      await new Promise(resolve => setTimeout(resolve, 100));
      
      const fileStructure = await invoke<FileNode>('get_wiki_file_structure', {
        wikiName: selectedWikiName.value
      });
      
      // 添加延迟让加载状态显示足够长时间
      await new Promise(resolve => setTimeout(resolve, 100));
      
      workspaceItems.value = fileStructure.children || [];

      // 重置文件夹展开状态
      const newExpandedStates: Record<string, boolean> = {};
      const initExpandedStates = (nodes: FileNode[]) => {
        nodes.forEach(node => {
          if (node.is_directory) {
            newExpandedStates[node.path] = false;
            if (node.children) {
              initExpandedStates(node.children);
            }
          }
        });
      };
      initExpandedStates(workspaceItems.value);
      folderExpandedStates.value = newExpandedStates;
    } catch (err) {
      console.error('Failed to refresh workspace:', err);
      workspaceError.value = err instanceof Error ? err.message : '获取文件结构失败';
      // 添加延迟让加载状态显示足够长时间
      await new Promise(resolve => setTimeout(resolve, 100));
    } finally {
      isWorkspaceLoading.value = false;
    }
  };

// 组件挂载时获取知识库列表并检查路由
onMounted(async () => {
  try {
    isLoading.value = true;
    const result = await invoke<Wiki[]>('get_wiki_list');
    wikis.value = result;

    // 检查当前路由
    const currentRoute = router.currentRoute.value;
    if (currentRoute.name !== 'home' && currentRoute.name !== 'workspace') {
      // 如果不是有效的路由，则导航到主页
      router.push('/');
    } else if (currentRoute.name === 'workspace') {
      // 如果是工作区路由，确保selectedWikiName与路由参数匹配
      const wikiName = currentRoute.params.wikiName as string;
      if (wikiName && !selectedWikiName.value) {
        selectedWikiName.value = wikiName;
        expandedWikiName.value = wikiName;
        // 加载对应的文件结构
        try {
          const fileStructure = await invoke<FileNode>('get_wiki_file_structure', { wikiName });
          workspaceItems.value = fileStructure.children || [];
          // 初始化文件夹展开状态
          const newExpandedStates: Record<string, boolean> = {};
          const initExpandedStates = (nodes: FileNode[]) => {
            nodes.forEach(node => {
              if (node.is_directory) {
                newExpandedStates[node.path] = false;
                if (node.children) {
                  initExpandedStates(node.children);
                }
              }
            });
          };
          initExpandedStates(workspaceItems.value);
          folderExpandedStates.value = newExpandedStates;
        } catch (err) {
          console.error('Failed to get wiki file structure:', err);
          alert('获取知识库文件结构失败');
        }
      }
    }
  } catch (err) {
    error.value = err instanceof Error ? err.message : '获取知识库列表失败';
    console.error('Failed to get wiki list:', err);
  } finally {
    isLoading.value = false;
  }
});

const selectedWikiName = ref<string | null>(null);
const expandedWikiName = ref<string | null>(null);
const router = useRouter();

// 存储每个文件夹的展开状态
const folderExpandedStates: Ref<Record<string, boolean>> = ref({});

// 切换侧边栏折叠状态
const toggleSidebarCollapse = () => {
  isSidebarCollapsed.value = !isSidebarCollapsed.value;
};

// 控制知识库和工作区折叠状态
const isWikiCollapsed = ref(false);
const isWorkspaceCollapsed = ref(false);

// 切换知识库折叠状态
const toggleWikiCollapse = () => {
  isWikiCollapsed.value = !isWikiCollapsed.value;
};

// 切换工作区折叠状态
const toggleWorkspaceCollapse = () => {
  isWorkspaceCollapsed.value = !isWorkspaceCollapsed.value;
};

const navigateTo = (path: string) => {
  if (path === '/') {
    // 清空选择的知识库状态和工作区文件结构
    selectedWikiName.value = null;
    expandedWikiName.value = null;
    workspaceItems.value = [];
  }
  router.push(path);
};

const selectWiki = async (wikiName: string) => {
  // 如果点击的是已选中的知识库，则切换展开/折叠状态
  if (selectedWikiName.value === wikiName) {
    expandedWikiName.value = expandedWikiName.value === wikiName ? null : wikiName;
  } else {
    selectedWikiName.value = wikiName;
    expandedWikiName.value = wikiName;
    
    // 获取知识库文件结构
    try {
      const fileStructure = await invoke<FileNode>('get_wiki_file_structure', { wikiName });
      workspaceItems.value = fileStructure.children || [];
      // 初始化文件夹展开状态
      const newExpandedStates: Record<string, boolean> = {};
      const initExpandedStates = (nodes: FileNode[]) => {
        nodes.forEach(node => {
          if (node.is_directory) {
            newExpandedStates[node.path] = false;
            if (node.children) {
              initExpandedStates(node.children);
            }
          }
        });
      };
      initExpandedStates(workspaceItems.value);
      folderExpandedStates.value = newExpandedStates;
    } catch (err) {
      console.error('Failed to get wiki file structure:', err);
      alert('获取知识库文件结构失败');
    }
    
    navigateTo(`/workspace/${wikiName}`);
  }
};

const createWiki = () => {
  isCreateWikiModalVisible.value = true;
};

const closeCreateWikiModal = () => {
  isCreateWikiModalVisible.value = false;
};

const onCreateWikiSuccess = async () => {
  isCreateWikiModalVisible.value = false;
  await refreshWikis();
};

const syncWiki = (wikiName: string) => {
  // 这里将在后续实现同步知识库的逻辑
  alert(`同步知识库 ${wikiName} 功能将在后续实现`);
};

const setupRemoteRepo = (wikiName: string) => {
  // 这里将在后续实现设置远程仓库的逻辑
  alert(`设置知识库 ${wikiName} 远程仓库功能将在后续实现`);
};

const deleteWiki = async (wikiName: string) => {
  // 弹出确认对话框
  confirm(`确定要删除知识库 "${wikiName}" 吗？此操作不可恢复！`);
  
  // 显示功能后续实现的提示
  alert(`删除知识库 ${wikiName} 功能将在后续实现`);
};

const createItem = () => {
  // 这里将在后续实现创建文件/文件夹的逻辑
  alert('创建文件/文件夹功能将在后续实现');
};

// 切换文件夹展开/收起状态
const toggleFolder = (node: FileNode) => {
  if (node.is_directory) {
    folderExpandedStates.value[node.path] = !folderExpandedStates.value[node.path];
  }
};

// 处理文件点击事件
const handleFileClick = (node: FileNode) => {
  if (!node.is_directory) {
    alert(`文件编辑功能将在后续实现`);
  }
};

</script>

<style scoped>
.refresh-btn {
  background: none;
  border: none;
  color: inherit;
  cursor: pointer;
  padding: 4px;
  margin-left: auto;
  opacity: 0.7;
  transition: opacity 0.2s;
}

.refresh-btn:hover {
  opacity: 1;
}

.group-title {
  display: flex;
  align-items: center;
}

.group-title span {
  font-weight: 600;
  color: #666;
}

.menu-toggle {
  cursor: pointer;
  transition: transform 0.3s ease;
}

.menu-toggle.collapsed {
  transform: rotate(90deg);
}

.sidebar {
  width: 250px;
  height: 100vh;
  background-color: #ffffff;
  border-right: 1px solid #eee;
  display: flex;
  flex-direction: column;
  transition: all 0.3s ease;
}

.sidebar.collapsed {
  width: 50px !important;
  overflow: hidden;
}

/* 当侧边栏收起时，隐藏所有除了展开图标的内容 */
.sidebar.collapsed .logo-container {
  padding: 15px 10px;
  display: flex;
  justify-content: center;
  align-items: center;
  border-bottom: none;
  height: 64px;
  box-sizing: border-box;
}

.sidebar.collapsed .logo {
  display: none;
}

.sidebar.collapsed .nav-menu {
  display: none;
}

.sidebar-toggle {
  cursor: pointer;
  transition: transform 0.3s ease;
  width: 24px !important;
  height: 24px !important;
  min-width: 24px;
  min-height: 24px;
  color: #666;
}

.sidebar .sidebar-toggle {
  margin-right: 15px;
}

.sidebar.collapsed .sidebar-toggle {
  margin-right: 0;
  display: flex;
  justify-content: center;
  align-items: center;
}

/* 移动端适配 */
@media (max-width: 768px) {
  /* 默认在移动端显示较窄的侧边栏 */
  .sidebar {
    width: 200px;
    padding-top: 20px; /* 在移动设备上，侧边栏内容下移20px */
  }
  
  /* 移动端完全收起侧边栏 */
  .sidebar.collapsed {
    width: 50px !important;
  }
}

.loading-item, .error-item, .empty-item {
  display: flex;
  align-items: center;
  padding: 10px 20px;
  color: #666;
  transition: opacity 0.3s ease-in-out;
}

.wiki-list-container {
  transition: opacity 0.3s ease-in-out;
}

.fade-enter-active, .fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from, .fade-leave-to {
  opacity: 0;
}

.loading-item .spinner {
  width: 16px;
  height: 16px;
  border: 2px solid #f3f3f3;
  border-top: 2px solid #3498db;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-right: 10px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.error-item {
  color: #e74c3c;
}

.empty-item {
  color: #999;
}

.logo-container {
  padding: 20px;
  border-bottom: 1px solid #eee;
  display: flex;
  align-items: center;
  height: 64px;
  box-sizing: border-box;
}

.logo {
  font-size: 24px;
  font-weight: bold;
  color: #000000;
  margin: 0;
}

.nav-menu {
  flex: 1;
  padding: 10px 0;
  overflow-y: auto;
  background-color: #ffffff;
}

.nav-group {
  margin-bottom: 15px;
}

.group-title {
  padding: 10px 20px;
  font-size: 12px;
  color: #000000;
  text-transform: uppercase;
  letter-spacing: 1px;
  font-weight: bold;
}

.wiki-item {
  position: relative;
}

.nav-item {
  display: flex;
  align-items: center;
  padding: 10px 20px;
  color: #000000;
  cursor: pointer;
  transition: background-color 0.2s;
}

.wiki-list-container {
  transition: all 0.3s ease;
}

.wiki-list-container.collapsed {
  max-height: 0;
  overflow: hidden;
}

.wiki-subitems {
  background-color: #f9f9f9;
  border-left: 2px solid #ddd;
}

.wiki-subitems .nav-item {
  padding-left: 40px;
}

.nav-item:hover {
  background-color: #f5f5f5;
}

.nav-item.active {
  background-color: #f0f0f0;
  font-weight: 500;
}

.icon {
  margin-right: 10px;
  width: 18px;
  height: 18px;
  color: #666;
}

.create-wiki {
  color: #000000;
  font-weight: 500;
}

.placeholder-item {
  color: #999;
  cursor: not-allowed;
}

.folder-icon {
  color: #666;
  font-size: 18px;
}

.file-icon {
  color: #666;
  font-size: 18px;
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

.child-item, .grandchild-item {
  padding-left: 40px;
}

.create-item {
  color: #000000;
  font-weight: 500;
}

/* 移除深色模式支持 */
</style>