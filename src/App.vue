<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { RouterView } from "vue-router";
import Sidebar from "./components/Sidebar.vue";

// 侧边栏宽度，默认为250px
const sidebarWidth = ref(250);
// 分隔条是否正在拖拽
const isDragging = ref(false);

// 处理鼠标按下事件，开始拖拽
const startDrag = (_: MouseEvent) => {
  isDragging.value = true;
  document.body.style.userSelect = 'none'; // 防止拖拽时选中文本
};

// 处理鼠标移动事件，调整侧边栏宽度
const handleDrag = (e: MouseEvent) => {
  if (!isDragging.value) return;
  // 确保侧边栏宽度在合理范围内
  const newWidth = Math.max(200, Math.min(e.clientX, window.innerWidth - 300));
  sidebarWidth.value = newWidth;
};

// 处理鼠标释放事件，结束拖拽
const endDrag = () => {
  isDragging.value = false;
  document.body.style.userSelect = '';
};

// 组件挂载时添加事件监听
onMounted(() => {
  window.addEventListener('mousemove', handleDrag);
  window.addEventListener('mouseup', endDrag);
});

// 组件卸载时移除事件监听
onUnmounted(() => {
  window.removeEventListener('mousemove', handleDrag);
  window.removeEventListener('mouseup', endDrag);
});
</script>

<template>
  <div class="app-container" @contextmenu.prevent>
    <Sidebar :style="{ width: sidebarWidth + 'px' }" />
    <div 
      class="resizer"
      @mousedown="startDrag"
      :class="{ 'dragging': isDragging }"
    />
    <div class="main-content" :style="{ width: `calc(100% - ${sidebarWidth + 5}px)` }">
      <RouterView />
    </div>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

</style>
<style>
:root {
  font-family: Inter, sans-serif;
  font-size: 16px;
  line-height: 1.5;
  font-weight: 400;

  color: #000000;
  background-color: #ffffff;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.app-container {
  display: flex;
  height: 100vh;
  background-color: #ffffff;
  overflow: hidden;
  position: relative;
}

/* 可拖拽分隔条样式 */
.resizer {
  width: 5px;
  background-color: #eee;
  cursor: col-resize;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.2s;
}

.resizer:hover {
  background-color: #ccc;
}

.resizer.dragging {
  background-color: #666;
}

/* 确保侧边栏内容不会溢出 */
.sidebar {
  overflow: hidden;
  transition: width 0.2s;
}

/* 隐藏body的滚动条 */
body {
  overflow: hidden;
  margin: 0;
  padding: 0;
}

.main-content {
  flex: 1;
  overflow-y: auto;
  background-color: #ffffff;
  color: #000000;
  /* 确保滚动条不超出容器 */
  max-height: 100%;
  /* 可选：自定义滚动条样式使其不那么明显 */
  scrollbar-width: thin;
  scrollbar-color: #ccc #f5f5f5;
}

/* 自定义滚动条 - WebKit浏览器 */
.main-content::-webkit-scrollbar {
  width: 6px;
}

.main-content::-webkit-scrollbar-track {
  background: #f5f5f5;
}

.main-content::-webkit-scrollbar-thumb {
  background-color: #ccc;
  border-radius: 3px;
}

a {
  color: #000000;
  text-decoration: none;
}

a:hover {
  text-decoration: underline;
}

h1 {
  margin-top: 0;
}

input,
button {
  border-radius: 4px;
  border: 1px solid #ccc;
  padding: 0.5em 1em;
  font-size: 1em;
  font-family: inherit;
  color: #000000;
  background-color: #ffffff;
  transition: all 0.2s;
}

button {
  cursor: pointer;
  background-color: #f8f8f8;
}

button:hover {
  background-color: #f0f0f0;
  border-color: #aaa;
}

button:active {
  background-color: #e8e8e8;
}

input:focus {
  border-color: #666;
  outline: none;
}

<template>
  <router-view />
</template>

<style>
/* 基础样式重置 */
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body, #app {
  height: 100%;
}

</style>