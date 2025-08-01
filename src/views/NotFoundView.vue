<template>
  <div class="not-found-container">
    <div class="not-found-content">
      <svg class="not-found-icon" xmlns="http://www.w3.org/2000/svg" width="80" height="80" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="12" cy="12" r="10"></circle>
        <line x1="12" x2="12" y1="8" y2="12"></line>
        <line x1="12" x2="12.01" y1="16" y2="16"></line>
      </svg>
      <h1>404</h1>
      <p>页面未找到</p>
      <button class="back-btn" @click="goBack">返回主页</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';

const router = useRouter();

const goBack = () => {
  router.push('/');
};
</script>

<style scoped>
.not-found-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
  width: 100%;
  background-color: #ffffff;
}

.not-found-content {
  text-align: center;
  height: 50%;
  width: 70%;
  padding: 2rem;
  border-radius: 8px;
  background-color: white;
  border: 1px solid #eee;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

.not-found-icon {
  margin-bottom: 1rem;
  color: #666;
}

h1 {
  font-size: 3rem;
  margin-bottom: 0.5rem;
  color: #000000;
}

p {
  font-size: 1.2rem;
  margin-bottom: 1.5rem;
  color: #333;
}

.back-btn {
  background-color: #f0f0f0;
  color: #000000;
  border: 1px solid #ddd;
  padding: 0.8rem 1.5rem;
  font-size: 1rem;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
}

.back-btn:hover {
  background-color: #e0e0e0;
  border-color: #ccc;
}

/* 移除深色模式支持 */
</style>